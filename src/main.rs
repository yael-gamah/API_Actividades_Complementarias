use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use rand::Rng;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool, FromRow};
use env_logger;

// Estructura para representar un estudiante
#[derive(Serialize, Deserialize, Debug, FromRow)]
struct Estudiante {
    numero_control: i32,
    nombre: String,
    apellido_paterno: String,
    apellido_materno: String,
    tipo_actividad: String,
    descripcion: String,
}

// Estructura para la solicitud de creación de un estudiante
#[derive(Serialize, Deserialize, Debug)]
struct EstudianteRequest {
    nombre: String,
    apellido_paterno: String,
    apellido_materno: String,
    tipo_actividad: String,
}

// Función para obtener la descripción basada en el tipo de actividad
fn obtener_descripcion(tipo: &str) -> String {
    match tipo {
        "Tutorias" => "Sesiones de asesoramiento con profesores.".to_string(),
        "DeportivasYCulturales" => "Actividades deportivas y culturales.".to_string(),
        "ProyectosDeInvestigacion" => "Participación en proyectos de investigación.".to_string(),
        "EventosAcademicos" => "Participación en eventos académicos.".to_string(),
        "ProductividadLaboral" => "Actividades de productividad laboral.".to_string(),
        "Emprendedurismo" => "Actividades de emprendimiento.".to_string(),
        "ProyectosInterdisciplinarios" => "Proyectos interdisciplinarios.".to_string(),
        "PrototiposYDesarrolloTecnologico" => "Desarrollo de prototipos tecnológicos.".to_string(),
        "MedioAmbiente" => "Actividades de conservación del medio ambiente.".to_string(),
        _ => "Actividad definida por el comité académico.".to_string(),
    }
}

// Handler para obtener todos los estudiantes desde la base de datos
async fn obtener_estudiantes(pool: web::Data<Pool<MySql>>) -> impl Responder {
    let estudiantes = sqlx::query_as::<_, Estudiante>(
        "SELECT numero_control, nombre, apellido_paterno, apellido_materno, tipo_actividad, descripcion FROM estudiantes"
    )
    .fetch_all(pool.get_ref())
    .await;

    match estudiantes {
        Ok(estudiantes) => HttpResponse::Ok().json(estudiantes),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Handler para crear un nuevo estudiante en la base de datos
async fn crear_estudiante(
    pool: web::Data<Pool<MySql>>,
    estudiante: web::Json<EstudianteRequest>,
) -> impl Responder {
    let descripcion = obtener_descripcion(&estudiante.tipo_actividad);

    let numero_control: i32 = rand::thread_rng().gen_range(10000000..99999999);

    let resultado = sqlx::query(
        r#"
        INSERT INTO estudiantes (numero_control, nombre, apellido_paterno, apellido_materno, tipo_actividad, descripcion)
        VALUES (?, ?, ?, ?, ?, ?)
        "#
    )
    .bind(numero_control)
    .bind(&estudiante.nombre)
    .bind(&estudiante.apellido_paterno)
    .bind(&estudiante.apellido_materno)
    .bind(&estudiante.tipo_actividad)
    .bind(&descripcion)
    .execute(pool.get_ref())
    .await;

    match resultado {
        Ok(_) => HttpResponse::Created().json("Estudiante creado"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Handler para actualizar un estudiante por número de control
async fn actualizar_estudiante(
    pool: web::Data<Pool<MySql>>,
    numero_control: web::Path<i32>,
    estudiante_actualizado: web::Json<EstudianteRequest>,
) -> impl Responder {
    let descripcion = obtener_descripcion(&estudiante_actualizado.tipo_actividad);

    let resultado = sqlx::query(
        r#"
        UPDATE estudiantes
        SET nombre = ?, apellido_paterno = ?, apellido_materno = ?, tipo_actividad = ?, descripcion = ?
        WHERE numero_control = ?
        "#
    )
    .bind(&estudiante_actualizado.nombre)
    .bind(&estudiante_actualizado.apellido_paterno)
    .bind(&estudiante_actualizado.apellido_materno)
    .bind(&estudiante_actualizado.tipo_actividad)
    .bind(&descripcion)
    .bind(numero_control.into_inner())
    .execute(pool.get_ref())
    .await;

    match resultado {
        Ok(_) => HttpResponse::Ok().json("Estudiante actualizado"),
        Err(_) => HttpResponse::NotFound().json("Estudiante no encontrado"),
    }
}

// Handler para eliminar un estudiante por número de control
async fn eliminar_estudiante(
    pool: web::Data<Pool<MySql>>,
    numero_control: web::Path<i32>,
) -> impl Responder {
    let resultado = sqlx::query(
        r#"
        DELETE FROM estudiantes WHERE numero_control = ?
        "#
    )
    .bind(numero_control.into_inner())
    .execute(pool.get_ref())
    .await;

    match resultado {
        Ok(_) => HttpResponse::Ok().json("Estudiante eliminado"),
        Err(_) => HttpResponse::NotFound().json("Estudiante no encontrado"),
    }
}

// Función principal para configurar y ejecutar el servidor
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Inicializar el logger
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Conectar a la base de datos MySQL
    let database_url = "mysql://root@localhost/Actividades_Complementarias";
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("No se pudo conectar a la base de datos");

    // Configuración del servidor con las rutas
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/actividades_complementarias", web::get().to(obtener_estudiantes))
            .route("/actividades_complementarias", web::post().to(crear_estudiante))
            .route("/actividades_complementarias/{numero_control}", web::put().to(actualizar_estudiante))
            .route("/actividades_complementarias/{numero_control}", web::delete().to(eliminar_estudiante))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}