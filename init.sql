-- Crear la base de datos si no existe
CREATE DATABASE IF NOT EXISTS Actividades_Complementarias;

-- Usar la base de datos
USE Actividades_Complementarias;

-- Crear la tabla estudiantes si no existe
CREATE TABLE IF NOT EXISTS estudiantes (
    numero_control INT PRIMARY KEY,
    nombre VARCHAR(50),
    apellido_paterno VARCHAR(50),
    apellido_materno VARCHAR(50),
    tipo_actividad VARCHAR(100),
    descripcion VARCHAR(255)
);

-- Insertar 10 registros de prueba en la tabla estudiantes
INSERT INTO estudiantes (numero_control, nombre, apellido_paterno, apellido_materno, tipo_actividad, descripcion) VALUES
(12345678, 'Juan', 'Pérez', 'Gómez', 'Tutorias', 'Sesiones de asesoramiento con profesores.'),
(23456789, 'María', 'López', 'Martínez', 'DeportivasYCulturales', 'Actividades deportivas y culturales.'),
(34567890, 'Carlos', 'Hernández', 'Ruiz', 'ProyectosDeInvestigacion', 'Participación en proyectos de investigación.'),
(45678901, 'Ana', 'Ramírez', 'Sánchez', 'EventosAcademicos', 'Participación en eventos académicos.'),
(56789012, 'Luis', 'García', 'Torres', 'ProductividadLaboral', 'Actividades de productividad laboral.'),
(67890123, 'Gabriela', 'Flores', 'Morales', 'Emprendedurismo', 'Actividades de emprendimiento.'),
(78901234, 'Miguel', 'Ríos', 'Vargas', 'ProyectosInterdisciplinarios', 'Proyectos interdisciplinarios.'),
(89012345, 'Laura', 'Jiménez', 'Ortega', 'PrototiposYDesarrolloTecnologico', 'Desarrollo de prototipos tecnológicos.'),
(90123456, 'Daniel', 'Castillo', 'Hernández', 'MedioAmbiente', 'Actividades de conservación del medio ambiente.'),
(10123456, 'Sofía', 'Mendoza', 'Aguilar', 'Tutorias', 'Sesiones de asesoramiento con profesores.');