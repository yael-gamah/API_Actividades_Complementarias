# TÓPICOS PARA EL DESPLIEGUE DE APLICACIONES.

## Creación de API REST utilizando RUST.

### Integrantes del equipo:

- Fernando de Jesús Barrón Mojica.

- Jesús Yael Gama Hernández.

- Joana Areli López Sánchez.

- Marcos Eduardo Hernández Moreno.

- Gael Alejandro Parra Herrera.

# API de Actividades Complementarias

Esta es una API RESTful desarrollada en Rust para gestionar actividades complementarias de estudiantes del Instituto Tecnológico de León, como tutorías, deportes, proyectos de investigación, entre otros. La API se ejecuta en un entorno Docker junto con una base de datos MySQL.

## Características

- CRUD para estudiantes y sus actividades complementarias.
- Asignación automática de descripciones para cada tipo de actividad.
- Implementación usando el framework Actix-web en Rust.
- Contenedorización con Docker para una implementación sencilla.

## Requisitos

- Docker (versión 1.82 o superior).
- Docker Compose.
- Git para clonar el repositorio.

## Instalación

### 1. Clonar el repositorio

   ```bash
   git clone <URL-DEL-REPOSITORIO>
   cd <NOMBRE-DEL-REPOSITORIO>
  ```
### 2. Construir y ejecutar la API con Docker

Asegúrate de estar en la raíz del proyecto y ejecuta:

```bash
docker-compose up --build
```
Esto construirá la imagen de la API y levantará los contenedores para la API y la base de datos.

### 4. Acceder a la API

La API estará disponible en [http://localhost:8080](http://localhost:8080). Puedes probar las rutas usando herramientas como Postman, cURL o tu navegador web.

#### Endpoints

##### Obtener todos los estudiantes
- **URL:** `/actividades_complementarias`
- **Método:** `GET`
- **Descripción:** Devuelve una lista de todos los estudiantes en la base de datos.

##### Crear un nuevo estudiante
- **URL:** `/actividades_complementarias`
- **Método:** `POST`
- **Cuerpo de la solicitud:** JSON con el siguiente formato:

```json
{
  "nombre": "Juan",
  "apellido_paterno": "Pérez",
  "apellido_materno": "López",
  "tipo_actividad": "Tutorias"
}
```
##### Actualizar un estudiante
- **URL:** `/actividades_complementarias/{numero_control}`
- **Método:** `PUT`
- **Cuerpo de la solicitud:** JSON con el siguiente formato:

```json
{
  "nombre": "Juan",
  "apellido_paterno": "Pérez",
  "apellido_materno": "López",
  "tipo_actividad": "ProyectosDeInvestigacion"
}
```
##### Eliminar un estudiante
- **URL:** `/actividades_complementarias/{numero_control}`
- **Método:** `DELETE`
- **Descripción:** Elimina el registro de un estudiante de la base de datos basado en su `numero_control`.








