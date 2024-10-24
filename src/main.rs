use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::sync::Mutex;
use log::info;
use env_logger;

// Enum para las actividades complementarias
#[derive(Serialize, Deserialize, Clone, Debug)]
enum ActividadComplementaria {
    Tutorias,
    DeportivasYCulturales,
    ProyectosDeInvestigacion,
    EventosAcademicos,
    ProductividadLaboral,
    Emprendedurismo,
    ProyectosInterdisciplinarios,
    PrototiposYDesarrolloTecnologico,
    MedioAmbiente,
    DefinidaPorComite(String), // Para actividades definidas por el comité
}

// Estructura para representar un estudiante
#[derive(Serialize, Deserialize, Clone, Debug)]
struct Estudiante {
    id: Uuid,
    nombre: String,
    apellido_paterno: String,
    apellido_materno: String,
    actividad: ActividadComplementaria,
}

// Estructura para la solicitud de creación de un estudiante
#[derive(Serialize, Deserialize, Debug)]
struct EstudianteRequest {
    nombre: String,
    apellido_paterno: String,
    apellido_materno: String,
    actividad: ActividadComplementaria,
}

// Estado compartido de la aplicación para almacenar estudiantes
struct EstadoApp {
    estudiantes: Mutex<Vec<Estudiante>>,
}

// Handler para obtener todos los estudiantes
async fn obtener_estudiantes(estado: web::Data<EstadoApp>) -> impl Responder {
    let estudiantes = estado.estudiantes.lock().unwrap();
    info!("Solicitud GET para obtener todos los estudiantes");
    HttpResponse::Ok().json(&*estudiantes)
}

// Handler para crear un nuevo estudiante
async fn crear_estudiante(estudiante: web::Json<EstudianteRequest>, estado: web::Data<EstadoApp>) -> impl Responder {
    let mut estudiantes = estado.estudiantes.lock().unwrap();

    let nuevo_estudiante = Estudiante {
        id: Uuid::new_v4(),
        nombre: estudiante.nombre.clone(),
        apellido_paterno: estudiante.apellido_paterno.clone(),
        apellido_materno: estudiante.apellido_materno.clone(),
        actividad: estudiante.actividad.clone(),
    };

    estudiantes.push(nuevo_estudiante.clone());
    info!("Estudiante creado: {:?}", nuevo_estudiante);
    HttpResponse::Created().json(nuevo_estudiante)
}

// Handler para actualizar un estudiante
async fn actualizar_estudiante(
    estudiante_id: web::Path<Uuid>,
    estudiante_actualizado: web::Json<EstudianteRequest>,
    estado: web::Data<EstadoApp>,
) -> impl Responder {
    let mut estudiantes = estado.estudiantes.lock().unwrap();
    if let Some(estudiante) = estudiantes.iter_mut().find(|e| e.id == *estudiante_id) {
        estudiante.nombre = estudiante_actualizado.nombre.clone();
        estudiante.apellido_paterno = estudiante_actualizado.apellido_paterno.clone();
        estudiante.apellido_materno = estudiante_actualizado.apellido_materno.clone();
        estudiante.actividad = estudiante_actualizado.actividad.clone();
        info!("Estudiante actualizado: {:?}", estudiante);
        HttpResponse::Ok().json(estudiante)
    } else {
        HttpResponse::NotFound().finish()
    }
}

// Handler para eliminar un estudiante
async fn eliminar_estudiante(estudiante_id: web::Path<Uuid>, estado: web::Data<EstadoApp>) -> impl Responder {
    let mut estudiantes = estado.estudiantes.lock().unwrap();
    if estudiantes.iter().any(|e| e.id == *estudiante_id) {
        estudiantes.retain(|e| e.id != *estudiante_id);
        info!("Estudiante eliminado con ID: {}", estudiante_id);
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

// Función principal para configurar y ejecutar el servidor
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Inicializar el logger
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Estado compartido de la aplicación
    let estado_app = web::Data::new(EstadoApp {
        estudiantes: Mutex::new(vec![]),
    });

    // Configuración del servidor
    HttpServer::new(move || {
        App::new()
            .app_data(estado_app.clone())
            .route("/estudiantes", web::get().to(obtener_estudiantes))
            .route("/estudiantes", web::post().to(crear_estudiante))
            .route("/estudiantes/{id}", web::put().to(actualizar_estudiante))
            .route("/estudiantes/{id}", web::delete().to(eliminar_estudiante))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}