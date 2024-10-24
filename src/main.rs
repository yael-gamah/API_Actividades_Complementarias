use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use rand::Rng;
use std::sync::Mutex;
use log::info;
use env_logger;

// Enum para los tipos de actividad complementaria
#[derive(Serialize, Deserialize, Clone, Debug)]
enum TipoActividad {
    Tutorias,
    DeportivasYCulturales,
    ProyectosDeInvestigacion,
    EventosAcademicos,
    ProductividadLaboral,
    Emprendedurismo,
    ProyectosInterdisciplinarios,
    PrototiposYDesarrolloTecnologico,
    MedioAmbiente,
    DefinidaPorComite(String),
}

// Estructura para representar la actividad complementaria con descripción
#[derive(Serialize, Deserialize, Clone, Debug)]
struct ActividadComplementaria {
    tipo: TipoActividad,
    descripcion: String,
}

// Función para obtener la descripción basada en el tipo de actividad
fn obtener_descripcion(tipo: &TipoActividad) -> String {
    match tipo {
        TipoActividad::Tutorias => "Sesiones de asesoramiento con profesores para apoyar el desempeño académico de los estudiantes.".to_string(),
        TipoActividad::DeportivasYCulturales => "Actividades deportivas y culturales para fomentar el desarrollo integral.".to_string(),
        TipoActividad::ProyectosDeInvestigacion => "Participación en proyectos de investigación orientados a generar nuevos conocimientos.".to_string(),
        TipoActividad::EventosAcademicos => "Participación en congresos, seminarios y otros eventos académicos.".to_string(),
        TipoActividad::ProductividadLaboral => "Actividades relacionadas con el desempeño en el ámbito laboral.".to_string(),
        TipoActividad::Emprendedurismo => "Fomento de actividades de emprendimiento y desarrollo de nuevas ideas de negocio.".to_string(),
        TipoActividad::ProyectosInterdisciplinarios => "Proyectos que involucran la colaboración de múltiples disciplinas.".to_string(),
        TipoActividad::PrototiposYDesarrolloTecnologico => "Construcción de prototipos y desarrollo de tecnologías innovadoras.".to_string(),
        TipoActividad::MedioAmbiente => "Actividades relacionadas con la conservación y protección del medio ambiente.".to_string(),
        TipoActividad::DefinidaPorComite(actividad) => format!("Actividad definida por el comité académico: {}", actividad),
    }
}

// Estructura para representar un estudiante
#[derive(Serialize, Deserialize, Clone, Debug)]
struct Estudiante {
    numero_control: u32,
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
    tipo_actividad: TipoActividad,
}

// Estado compartido de la aplicación para almacenar estudiantes
struct EstadoApp {
    estudiantes: Mutex<Vec<Estudiante>>,
}

// Función para generar un número de control de 8 dígitos
fn generar_numero_control() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(10000000..99999999)
}

// Handler para obtener todos los estudiantes
async fn obtener_estudiantes(estado: web::Data<EstadoApp>) -> impl Responder {
    let estudiantes = estado.estudiantes.lock().unwrap();
    info!("Solicitud GET para obtener todas las actividades complementarias");
    HttpResponse::Ok().json(&*estudiantes)
}

// Handler para crear un nuevo estudiante
async fn crear_estudiante(estudiante: web::Json<EstudianteRequest>, estado: web::Data<EstadoApp>) -> impl Responder {
    let mut estudiantes = estado.estudiantes.lock().unwrap();

    // Crear la actividad complementaria con descripción automática
    let actividad_complementaria = ActividadComplementaria {
        tipo: estudiante.tipo_actividad.clone(),
        descripcion: obtener_descripcion(&estudiante.tipo_actividad),
    };

    let nuevo_estudiante = Estudiante {
        numero_control: generar_numero_control(),
        nombre: estudiante.nombre.clone(),
        apellido_paterno: estudiante.apellido_paterno.clone(),
        apellido_materno: estudiante.apellido_materno.clone(),
        actividad: actividad_complementaria,
    };

    estudiantes.push(nuevo_estudiante.clone());
    info!("Estudiante creado: {:?}", nuevo_estudiante);
    HttpResponse::Created().json(nuevo_estudiante)
}

// (Los demás handlers no cambian)

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

    // Configuración del servidor con la nueva ruta
    HttpServer::new(move || {
        App::new()
            .app_data(estado_app.clone())
            .route("/actividades_complementarias", web::get().to(obtener_estudiantes))
            .route("/actividades_complementarias", web::post().to(crear_estudiante))
            .route("/actividades_complementarias/{numero_control}", web::put().to(actualizar_estudiante))
            .route("/actividades_complementarias/{numero_control}", web::delete().to(eliminar_estudiante))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
