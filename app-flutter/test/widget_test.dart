import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';

import 'package:app_flutter/main.dart';
import 'package:app_flutter/core/bridge_runtime_config.dart';
import 'package:app_flutter/core/locale_controller.dart';
import 'package:app_flutter/features/patient_home/exercise_video_panel.dart';
import 'package:app_flutter/features/patient_home/patient_home_page.dart';
import 'package:app_flutter/features/patient_home/program_selection.dart';
import 'package:app_flutter/l10n/app_localizations.dart';
import 'package:app_flutter/shared/utils/youtube.dart';
import 'package:app_flutter/src/rust/api.dart' as rust_api;

void main() {
  Widget wrapWithL10n(Widget home) {
    return MaterialApp(
      locale: const Locale('en'),
      supportedLocales: AppLocalizations.supportedLocales,
      localizationsDelegates: AppLocalizations.localizationsDelegates,
      home: home,
    );
  }

  test('builds YouTube embed URL from watch URL', () {
    expect(
      buildYouTubeEmbedUrl('www.youtube.com/watch?v=dQw4w9WgXcQ'),
      'https://www.youtube.com/embed/dQw4w9WgXcQ',
    );
  });

  test('opens watch URLs from embedded player externally', () {
    expect(
      shouldOpenEmbeddedVideoNavigationExternally(
        'https://www.youtube.com/watch?v=dQw4w9WgXcQ',
      ),
      isTrue,
    );
  });

  test('does not open embed URLs from embedded player externally', () {
    expect(
      shouldOpenEmbeddedVideoNavigationExternally(
        'https://www.youtube.com/embed/dQw4w9WgXcQ',
      ),
      isFalse,
    );
  });

  test('linux desktop uses placeholder mode for exercise videos', () {
    expect(
      chooseExerciseVideoRenderMode(
        isTestEnv: false,
        isAndroid: false,
        isIos: false,
        isLinux: true,
      ),
      ExerciseVideoRenderMode.placeholder,
    );
  });

  test('auto-selects next pending training day', () {
    expect(
      choosePreferredTrainingDayIndex([
        const rust_api.ProgramDaySummary(
          dayIndex: 0,
          dayNumber: 1,
          workoutName: 'A',
          workoutDescription: null,
          isRestDay: false,
          sessionDate: '2026-03-15',
          completedAt: '2026-03-15T10:00:00Z',
          exercises: [],
        ),
        const rust_api.ProgramDaySummary(
          dayIndex: 1,
          dayNumber: 2,
          workoutName: null,
          workoutDescription: null,
          isRestDay: true,
          sessionDate: null,
          completedAt: null,
          exercises: [],
        ),
        const rust_api.ProgramDaySummary(
          dayIndex: 2,
          dayNumber: 3,
          workoutName: 'B',
          workoutDescription: null,
          isRestDay: false,
          sessionDate: '2026-03-17',
          completedAt: null,
          exercises: [],
        ),
      ]),
      2,
    );
  });

  test('auto-selects last training day when all are completed', () {
    expect(
      choosePreferredTrainingDayIndex([
        const rust_api.ProgramDaySummary(
          dayIndex: 0,
          dayNumber: 1,
          workoutName: 'A',
          workoutDescription: null,
          isRestDay: false,
          sessionDate: '2026-03-15',
          completedAt: '2026-03-15T10:00:00Z',
          exercises: [],
        ),
        const rust_api.ProgramDaySummary(
          dayIndex: 1,
          dayNumber: 2,
          workoutName: null,
          workoutDescription: null,
          isRestDay: true,
          sessionDate: null,
          completedAt: null,
          exercises: [],
        ),
        const rust_api.ProgramDaySummary(
          dayIndex: 2,
          dayNumber: 3,
          workoutName: 'B',
          workoutDescription: null,
          isRestDay: false,
          sessionDate: '2026-03-17',
          completedAt: '2026-03-17T10:00:00Z',
          exercises: [],
        ),
      ]),
      2,
    );
  });

  testWidgets('renders splash then login shell', (WidgetTester tester) async {
    await tester.pumpWidget(
      wrapWithL10n(
        const EixeApp(
          bridgeConfig: BridgeRuntimeConfig(
            supabaseUrl: 'https://example.supabase.co',
            supabaseAnonKey: 'anon-key',
          ),
          autoInitializeBridge: false,
        ),
      ),
    );

    expect(find.text('Eixe'), findsWidgets);
    expect(find.text('Starting app…'), findsOneWidget);
    expect(find.text('Sign in'), findsNothing);

    await tester.tap(find.text('Continue'));
    await tester.pumpAndSettle();

    expect(find.text('Welcome back'), findsOneWidget);
    expect(find.text('Sign in'), findsOneWidget);
    expect(find.text('Email'), findsOneWidget);
    expect(find.text('Password'), findsOneWidget);
  });

  testWidgets('renders patient home with selectable program detail', (
    WidgetTester tester,
  ) async {
    await tester.pumpWidget(
      wrapWithL10n(
        PatientHomePage(
          loginResponse: rust_api.LoginResponse(
            accessToken: 'token',
            userId: 'patient-1',
            userProfileType: 'patient',
          ),
          patientPrograms: [
            rust_api.PatientProgramSummary(
              patientProgramId: 'assignment-1',
              programId: 'program-1',
              programName: 'Recovery Basics',
              programDescription: 'Mobility and breathing work.',
              progressPercent: 50,
              averageEffort: 4.0,
              averagePain: 2.0,
              days: [
                rust_api.ProgramDaySummary(
                  dayIndex: 0,
                  dayNumber: 1,
                  workoutName: 'Breathing Flow',
                  workoutDescription: 'Gentle breathing and mobility.',
                  isRestDay: false,
                  sessionDate: '2026-03-15',
                  completedAt: '2026-03-15T10:00:00Z',
                  exercises: [
                    rust_api.ExerciseInstructionSummary(
                      exerciseId: 'exercise-1',
                      name: 'Diaphragmatic breathing',
                      description: 'Lie down and breathe slowly.',
                      videoUrl: 'www.youtube.com/watch?v=dQw4w9WgXcQ',
                      sets: 3,
                      reps: 10,
                      effort: 4,
                      pain: 2,
                      comment: 'Comfortable session.',
                    ),
                  ],
                ),
                rust_api.ProgramDaySummary(
                  dayIndex: 1,
                  dayNumber: 2,
                  workoutName: null,
                  workoutDescription: null,
                  isRestDay: true,
                  sessionDate: '2026-03-16',
                  completedAt: null,
                  exercises: [],
                ),
              ],
            ),
            rust_api.PatientProgramSummary(
              patientProgramId: 'assignment-2',
              programId: 'program-2',
              programName: 'Strength Builder',
              programDescription: 'Progressive lower-body exercises.',
              progressPercent: 0,
              averageEffort: null,
              averagePain: null,
              days: [
                rust_api.ProgramDaySummary(
                  dayIndex: 0,
                  dayNumber: 1,
                  workoutName: 'Strength Circuit',
                  workoutDescription: 'Squats, bridges, and calf raises.',
                  isRestDay: false,
                  sessionDate: '2026-03-17',
                  completedAt: null,
                  exercises: [
                    rust_api.ExerciseInstructionSummary(
                      exerciseId: 'exercise-2',
                      name: 'Bodyweight squat',
                      description: 'Keep your chest up.',
                      videoUrl: null,
                      sets: 4,
                      reps: 12,
                      effort: null,
                      pain: null,
                      comment: null,
                    ),
                  ],
                ),
              ],
            ),
          ],
          localeController: LocaleController(),
          localeLoaded: true,
        ),
      ),
    );

    expect(find.text('Your programs'), findsOneWidget);
    expect(find.text('2 assigned'), findsOneWidget);
    expect(find.text('Recovery Basics'), findsWidgets);
    expect(find.text('Mobility and breathing work.'), findsWidgets);
    expect(find.text('Day 1'), findsWidgets);
    expect(find.text('2026-03-15 • Completed'), findsOneWidget);
    expect(find.text('Diaphragmatic breathing'), findsOneWidget);
    expect(find.text('Lie down and breathe slowly.'), findsOneWidget);
    expect(find.text('3 sets · 10 reps'), findsOneWidget);
    expect(find.byKey(const Key('exercise-video-exercise-1')), findsOneWidget);
    expect(find.text('Exercise video'), findsOneWidget);
    expect(find.text('Video unavailable'), findsNothing);
    expect(find.text('Effort'), findsOneWidget);
    expect(find.text('4/10'), findsOneWidget);
    expect(find.text('Pain'), findsOneWidget);
    expect(find.text('2/10'), findsOneWidget);
    expect(find.text('Comment (optional)'), findsOneWidget);
    expect(find.text('Save'), findsOneWidget);
    expect(find.text('Save feedback'), findsNothing);
    expect(find.text('Save as completed'), findsNothing);
    expect(find.text('Mark as completed'), findsNothing);
    expect(find.text('Mark as not completed'), findsOneWidget);
    expect(find.text('Completion date'), findsOneWidget);
    expect(find.byKey(const Key('completion-date-field')), findsOneWidget);
    expect(find.byType(Slider), findsNWidgets(2));
    expect(find.byType(ChoiceChip), findsNothing);
    expect(find.text('Program details'), findsNothing);
    expect(find.textContaining('Assignment ID:'), findsNothing);
    expect(find.textContaining('Program ID:'), findsNothing);

    await tester.tap(find.text('Strength Builder').first);
    await tester.pumpAndSettle();

    expect(find.text('Progressive lower-body exercises.'), findsWidgets);
    expect(find.text('Bodyweight squat'), findsOneWidget);
    expect(find.text('Keep your chest up.'), findsOneWidget);
    expect(find.text('4 sets · 12 reps'), findsOneWidget);
    expect(find.byKey(const Key('exercise-video-exercise-2')), findsNothing);
    expect(find.text('Planned'), findsOneWidget);
    expect(find.text('Completion date'), findsOneWidget);
    expect(find.byKey(const Key('completion-date-field')), findsOneWidget);
    expect(find.text('Save'), findsNothing);
    expect(find.text('Save feedback'), findsNothing);
    expect(find.text('Save as completed'), findsOneWidget);
    expect(find.text('Mark as completed'), findsNothing);
    expect(find.text('Mark as not completed'), findsNothing);
    expect(find.textContaining('Rest'), findsNothing);
    expect(find.byType(Slider), findsNWidgets(2));
  });
}
