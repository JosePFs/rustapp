# Cómo asignar el rol (specialist / patient)

El rol se guarda en **User Metadata** al crear el usuario. El trigger `handle_new_user` lo lee y lo escribe en `profiles.role`.

## Valores válidos

| Rol         | Valor en metadata |
|------------|-------------------|
| Especialista | `"specialist"`   |
| Paciente   | `"patient"` (o no poner nada) |

---

## 1. Desde el Dashboard de Supabase

1. Ve a **Authentication** → **Users** → **Add user** (o **Invite**).
2. Rellena **Email** y **Password**.
3. En **User Metadata** (campo JSON) escribe:

   **Especialista:**
   ```json
   {"role": "specialist"}
   ```

   **Paciente:**
   ```json
   {"role": "patient"}
   ```
   O simplemente `{}` o deja el campo vacío (por defecto será paciente).

4. Crea el usuario. El trigger creará la fila en `profiles` con ese rol.

---

## 2. Desde la API (signUp)

Al registrar con la API de Auth, envía `user_metadata` en el cuerpo:

**Especialista:**
```json
{
  "email": "especialista@ejemplo.com",
  "password": "tu_password",
  "user_metadata": {
    "role": "specialist",
    "full_name": "Dr. García"
  }
}
```

**Paciente:**
```json
{
  "email": "paciente@ejemplo.com",
  "password": "tu_password",
  "user_metadata": {
    "role": "patient",
    "full_name": "María López"
  }
}
```

Si no envías `role` en `user_metadata`, el trigger usará `"patient"` por defecto.

---

## 3. Cambiar el rol de un usuario ya creado

Si el usuario ya existe, el trigger no se vuelve a ejecutar. Hay que actualizar `profiles` en la base de datos:

```sql
UPDATE public.profiles
SET role = 'specialist'   -- o 'patient'
WHERE email = 'usuario@ejemplo.com';
```

(O por `id` si tienes el UUID del usuario en Auth.)
