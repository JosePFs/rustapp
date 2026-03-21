-- RPC: get_workout_with_exercises
-- Returns workout + exercises array in a single call.
CREATE OR REPLACE FUNCTION public.get_workout_with_exercises(p_workout_id uuid)
RETURNS jsonb
LANGUAGE sql
STABLE
SECURITY INVOKER
SET search_path = public
AS $$
    SELECT jsonb_build_object(
        'workout', jsonb_build_object(
            'id', w.id,
            'specialist_id', w.specialist_id,
            'name', w.name,
            'description', w.description,
            'order_index', w.order_index,
            'created_at', w.created_at,
            'updated_at', w.updated_at
        ),
        'exercises', COALESCE((
            SELECT jsonb_agg(
                jsonb_build_object(
                    'order_index', we.order_index,
                    'sets', we.sets,
                    'reps', we.reps,
                    'exercise', jsonb_build_object(
                        'id', e.id,
                        'specialist_id', e.specialist_id,
                        'name', e.name,
                        'description', e.description,
                        'order_index', e.order_index,
                        'video_url', e.video_url,
                        'deleted_at', e.deleted_at,
                        'created_at', e.created_at
                    )
                ) ORDER BY we.order_index
            )
            FROM workout_exercises we
            JOIN exercises e ON e.id = we.exercise_id
            WHERE we.workout_id = w.id
        ), '[]'::jsonb)
    )
    FROM workouts w
    WHERE w.id = p_workout_id;
$$;

COMMENT ON FUNCTION public.get_workout_with_exercises(uuid) IS
    'Returns workout with all exercises embedded in a single JSONB response.';

GRANT EXECUTE ON FUNCTION public.get_workout_with_exercises(uuid) TO authenticated;

-- RPC: get_program_with_agenda
-- Returns program + schedule + workouts (with exercises) in a single call.
CREATE OR REPLACE FUNCTION public.get_program_with_agenda(p_program_id uuid)
RETURNS jsonb
LANGUAGE sql
STABLE
SECURITY INVOKER
SET search_path = public
AS $$
    SELECT jsonb_build_object(
        'program', jsonb_build_object(
            'id', p.id,
            'specialist_id', p.specialist_id,
            'name', p.name,
            'description', p.description
        ),
        'schedule', COALESCE((
            SELECT jsonb_agg(
                jsonb_build_object(
                    'id', ps.id,
                    'program_id', ps.program_id,
                    'order_index', ps.order_index,
                    'workout_id', ps.workout_id,
                    'days_count', ps.days_count,
                    'created_at', ps.created_at
                ) ORDER BY ps.order_index
            )
            FROM program_schedule ps
            WHERE ps.program_id = p.id
        ), '[]'::jsonb),
        'workouts', COALESCE((
            SELECT jsonb_agg(DISTINCT
                jsonb_build_object(
                    'workout', jsonb_build_object(
                        'id', w.id,
                        'specialist_id', w.specialist_id,
                        'name', w.name,
                        'description', w.description,
                        'order_index', w.order_index,
                        'created_at', w.created_at,
                        'updated_at', w.updated_at
                    ),
                    'exercises', COALESCE((
                        SELECT jsonb_agg(
                            jsonb_build_object(
                                'order_index', we.order_index,
                                'sets', we.sets,
                                'reps', we.reps,
                                'exercise', jsonb_build_object(
                                    'id', e.id,
                                    'specialist_id', e.specialist_id,
                                    'name', e.name,
                                    'description', e.description,
                                    'order_index', e.order_index,
                                    'video_url', e.video_url,
                                    'deleted_at', e.deleted_at,
                                    'created_at', e.created_at
                                )
                            ) ORDER BY we.order_index
                        )
                        FROM workout_exercises we
                        JOIN exercises e ON e.id = we.exercise_id
                        WHERE we.workout_id = w.id
                    ), '[]'::jsonb)
                )
            )
            FROM program_schedule ps2
            JOIN workouts w ON w.id = ps2.workout_id
            WHERE ps2.program_id = p.id AND ps2.workout_id IS NOT NULL
        ), '[]'::jsonb)
    )
    FROM programs p
    WHERE p.id = p_program_id;
$$;

COMMENT ON FUNCTION public.get_program_with_agenda(uuid) IS
    'Returns program with schedule and all workouts (with exercises) in a single JSONB response.';

GRANT EXECUTE ON FUNCTION public.get_program_with_agenda(uuid) TO authenticated;

-- RPC: get_patient_program_full
-- Returns complete patient program data for mobile/patient views.
CREATE OR REPLACE FUNCTION public.get_patient_program_full(p_patient_program_id uuid)
RETURNS jsonb
LANGUAGE sql
STABLE
SECURITY INVOKER
SET search_path = public
AS $$
    SELECT jsonb_build_object(
        'patient_program', jsonb_build_object(
            'id', pp.id,
            'patient_id', pp.patient_id,
            'program_id', pp.program_id,
            'status', pp.status
        ),
        'program', jsonb_build_object(
            'id', prog.id,
            'specialist_id', prog.specialist_id,
            'name', prog.name,
            'description', prog.description
        ),
        'schedule', COALESCE((
            SELECT jsonb_agg(
                jsonb_build_object(
                    'id', ps.id,
                    'program_id', ps.program_id,
                    'order_index', ps.order_index,
                    'workout_id', ps.workout_id,
                    'days_count', ps.days_count,
                    'created_at', ps.created_at
                ) ORDER BY ps.order_index
            )
            FROM program_schedule ps
            WHERE ps.program_id = pp.program_id
        ), '[]'::jsonb),
        'workouts', COALESCE((
            SELECT jsonb_agg(DISTINCT
                jsonb_build_object(
                    'workout', jsonb_build_object(
                        'id', w.id,
                        'specialist_id', w.specialist_id,
                        'name', w.name,
                        'description', w.description,
                        'order_index', w.order_index,
                        'created_at', w.created_at,
                        'updated_at', w.updated_at
                    ),
                    'exercises', COALESCE((
                        SELECT jsonb_agg(
                            jsonb_build_object(
                                'order_index', we.order_index,
                                'sets', we.sets,
                                'reps', we.reps,
                                'exercise', jsonb_build_object(
                                    'id', e.id,
                                    'specialist_id', e.specialist_id,
                                    'name', e.name,
                                    'description', e.description,
                                    'order_index', e.order_index,
                                    'video_url', e.video_url,
                                    'deleted_at', e.deleted_at,
                                    'created_at', e.created_at
                                )
                            ) ORDER BY we.order_index
                        )
                        FROM workout_exercises we
                        JOIN exercises e ON e.id = we.exercise_id
                        WHERE we.workout_id = w.id
                    ), '[]'::jsonb)
                )
            )
            FROM program_schedule ps2
            JOIN workouts w ON w.id = ps2.workout_id
            WHERE ps2.program_id = pp.program_id AND ps2.workout_id IS NOT NULL
        ), '[]'::jsonb),
        'sessions', COALESCE((
            SELECT jsonb_agg(
                jsonb_build_object(
                    'id', ws.id,
                    'patient_program_id', ws.patient_program_id,
                    'day_index', ws.day_index,
                    'session_date', ws.session_date,
                    'completed_at', ws.completed_at,
                    'created_at', ws.created_at,
                    'updated_at', ws.updated_at
                ) ORDER BY ws.day_index
            )
            FROM workout_sessions ws
            WHERE ws.patient_program_id = pp.id
        ), '[]'::jsonb),
        'feedback', COALESCE((
            SELECT jsonb_agg(
                jsonb_build_object(
                    'workout_session_id', sef.workout_session_id,
                    'exercise_id', sef.exercise_id,
                    'effort', sef.effort,
                    'pain', sef.pain,
                    'comment', sef.comment
                )
            )
            FROM session_exercise_feedback sef
            JOIN workout_sessions ws2 ON ws2.id = sef.workout_session_id
            WHERE ws2.patient_program_id = pp.id
        ), '[]'::jsonb)
    )
    FROM patient_programs pp
    JOIN programs prog ON prog.id = pp.program_id
    WHERE pp.id = p_patient_program_id;
$$;

COMMENT ON FUNCTION public.get_patient_program_full(uuid) IS
    'Returns complete patient program data including program, schedule, workouts, sessions, and feedback.';

GRANT EXECUTE ON FUNCTION public.get_patient_program_full(uuid) TO authenticated;

-- RPC: get_specialist_dashboard
-- Returns specialist dashboard data (links, profiles, programs, assignments).
CREATE OR REPLACE FUNCTION public.get_specialist_dashboard(p_specialist_id uuid)
RETURNS jsonb
LANGUAGE sql
STABLE
SECURITY INVOKER
SET search_path = public
AS $$
    SELECT jsonb_build_object(
        'links', COALESCE((
            SELECT jsonb_agg(
                jsonb_build_object(
                    'id', sp.id,
                    'specialist_id', sp.specialist_id,
                    'patient_id', sp.patient_id,
                    'created_at', sp.created_at
                )
            )
            FROM specialist_patients sp
            WHERE sp.specialist_id = p_specialist_id
        ), '[]'::jsonb),
        'profiles', COALESCE((
            SELECT jsonb_agg(
                jsonb_build_object(
                    'id', prof.id,
                    'email', prof.email,
                    'full_name', prof.full_name,
                    'role', prof.role,
                    'created_at', prof.created_at,
                    'updated_at', prof.updated_at
                )
            )
            FROM profiles prof
            WHERE prof.id IN (
                SELECT sp2.patient_id FROM specialist_patients sp2
                WHERE sp2.specialist_id = p_specialist_id
            )
        ), '[]'::jsonb),
        'programs', COALESCE((
            SELECT jsonb_agg(
                jsonb_build_object(
                    'id', prog.id,
                    'specialist_id', prog.specialist_id,
                    'name', prog.name,
                    'description', prog.description
                ) ORDER BY prog.created_at DESC
            )
            FROM programs prog
            WHERE prog.specialist_id = p_specialist_id
        ), '[]'::jsonb),
        'assignments', COALESCE((
            SELECT jsonb_agg(
                jsonb_build_object(
                    'id', pp.id,
                    'patient_id', pp.patient_id,
                    'program_id', pp.program_id,
                    'status', pp.status
                ) ORDER BY pp.assigned_at DESC
            )
            FROM patient_programs pp
            WHERE pp.patient_id IN (
                SELECT sp3.patient_id FROM specialist_patients sp3
                WHERE sp3.specialist_id = p_specialist_id
            )
        ), '[]'::jsonb)
    );
$$;

COMMENT ON FUNCTION public.get_specialist_dashboard(uuid) IS
    'Returns specialist dashboard data: linked patients with profiles, programs, and assignments.';

GRANT EXECUTE ON FUNCTION public.get_specialist_dashboard(uuid) TO authenticated;
