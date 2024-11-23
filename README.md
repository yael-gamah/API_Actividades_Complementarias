# TÓPICOS PARA EL DESPLIEGUE DE APLICACIONES.

## Creación de API REST utilizando RUST.

### Integrantes del equipo:

- Fernando de Jesús Barrón Mojica.

- Jesús Yael Gama Hernández.

- Joana Areli López Sánchez.

- Marcos Eduardo Hernández Moreno.

- Gael Alejandro Parra Herrera.

# API de Actividades Complementarias

Esta es una API REST desarrollada en Rust para gestionar actividades complementarias de estudiantes del Instituto Tecnológico de León, como tutorías, deportes, proyectos de investigación, entre otros. La API se ejecuta en un entorno Docker junto con una base de datos MySQL.

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
   git clone https://github.com/yael-gamah/API_Actividades_Complementarias.git
   cd API_Actividades_Complementarias
  ```
### 2. Construir y ejecutar la API con Docker

Asegúrate de estar en la raíz del proyecto y ejecuta:

```bash
docker-compose up --build
```
Esto construirá la imagen de la API y levantará los contenedores para la API y la base de datos.

### 3. Acceder a la API

La API estará disponible en [http://localhost:5050](http://localhost:5050). Puedes probar las rutas usando herramientas como Postman, cURL o tu navegador web.

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


# Pruebas de la API en Jenkins

### 1. Iniciar Jenkins en un Contenedor Docker

Ejecuta el siguiente comando en la terminal de PowerShell (Windows) para iniciar Jenkins en un contenedor Docker:

```bash
docker run --rm -u root -p 8080:8080 \
  -v jenkins-data:/var/jenkins_home \
  -v /var/run/docker.sock:/var/run/docker.sock \
  -v ${HOME}:/home \
  --name jenkins_server jenkins/jenkins:lts
```

![Build Successful](https://drive.google.com/uc?export=view&id=1LqE2TOtZTGDOUtPkBe452ZCk09LXBBAD)

## 2. Acceder a Jenkins
Una vez iniciado Jenkins, abre tu navegador web y ve a la URL [http://localhost:8080](http://localhost:8080). Inicia sesión con el usuario y contraseña de administrador.

![Inicio Jenkins](https://drive.google.com/uc?id=1voL3BjQrJnZjx9pDZeaHsvCu88QNDu7_)

## 3. Crear un Nuevo Proyecto en Jenkins
1. Desde el panel de control de Jenkins, haz clic en “Nuevo Tarea”.
2. Asigna un nombre al proyecto, como "BuildApi", y selecciona **Crear un proyecto de estilo libre**.
3. Haz clic en "OK" para crear el proyecto.

![Crear proyecto](https://drive.google.com/uc?id=1rhmBBtmAwTEqSRLjLaddKwOsCTJVWWwZ)

## 4. Configurar el Proyecto en Jenkins

### Descripción del Proyecto
En la sección "General", añade una descripción al proyecto, como "API de Actividades Complementarias", para facilitar su identificación.

![Agregar descripción](https://drive.google.com/uc?id=1SrqlqQ6ba5RL5BDbkxW1R_c2z_X119gW)

### Configurar el Código Fuente
1. En la sección **Origen del Código Fuente**, selecciona la opción **Git**.
2. En el campo "Repository URL", ingresa la URL del repositorio de la API en GitHub: [https://github.com/yael-gamah/API_Actividades_Complementarias.git](https://github.com/yael-gamah/API_Actividades_Complementarias.git)
3. Añade las credenciales necesarias para acceder al repositorio.

![Configuración código](https://drive.google.com/uc?id=1TGP-KkBXwljmxeUu32zHB_rwacmzQp93)

4. Especifica la rama a construir. En este caso, utiliza `*/main` para indicar la rama principal.

![Rama main](https://drive.google.com/uc?id=1P6x_JtgrXaJolgySs2a505dkebO2LOzU)

### Añadir un Paso de Construcción
1. En la sección **Construir**, añade un nuevo paso para **Ejecutar línea de comandos (shell)**.
2. Añade los siguientes comandos para construir la imagen Docker y levantar los contenedores:

```bash
# Construir la imagen Docker
docker-compose build

# Levantar los contenedores
docker-compose up -d
```
![Construcción](https://drive.google.com/uc?id=1phu9oY2hEfSg9ZrBpvYB8HFKqSAVnxPJ)

3. Haz clic en Guardar para aplicar los cambios.
   
## 5. Ejecutar el Proyecto en Jenkins
1. En el panel del proyecto, haz clic en **Construir ahora** para iniciar la compilación y despliegue de la API.

![Construir ahora](https://drive.google.com/uc?id=1RfAiCes9HueaivFIo9Gn785cEYC0DQmP)

2. Ve a la **Salida de Consola** para monitorear el progreso y asegurarte de que la compilación y el despliegue se realicen sin problemas.

![Salida de consola](https://drive.google.com/uc?id=1zyK6Xx86153SkuP_aNiQr3ifKxB3Kg8P)
![Salida de consola](https://drive.google.com/uc?id=1bWxbR29ar3wEPHBY6QxyzAtQ_2xE-sDH)

## 6. Verificar la Respuesta de la API en el Navegador:
Para confirmar que la API esté respondiendo correctamente, abre tu navegador web y visita la siguiente URL: [http://localhost:5050/actividades_complementarias](http://localhost:5050/actividades_complementarias)

Deberías ver una respuesta en formato JSON con la lista de estudiantes y sus actividades, similar a la siguiente:

![Respuesta API](https://drive.google.com/uc?id=1_GPY0brJoh537RsyHxhqY1TuDlIXE-Zv)

# Despliegue del API en kubernetes

## Aplica tus archivos de configuración

### En la terminal de comandos, y ubicados en la carpeta raíz del proyecto, aplica el archivo deployment.yaml para desplegar la aplicación y la base de datos.

```bash
kubectl apply -f deployment.yaml
```

### Puedes verificar que los pods están corriendo con:
```bash
kubectl get pods
```

## Exponer tu servicio

### Si necesitas acceder a tu aplicación desde el navegador o Postman, debes exponer el servicio. Usa este comando para obtener la URL del servicio expuesto:

```bash
minikube service api-service --url
```
### Esto mostrará algo como:

```bash
[minikube service api-service --url](http://127.0.0.1:53030)
```
## Verifica el estado de los Pods y Servicios

### Asegúrate de que todos los pods estén en estado Running y el servicio esté correctamente configurado:

```bash
kubectl get pods
kubectl get services
```
