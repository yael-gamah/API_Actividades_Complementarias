# Usa la imagen completa de Rust para la construcción
FROM rust:1.82

# Instala las dependencias necesarias para la conexión a MySQL
#RUN apt-get update && \
 #   apt-get install -y libssl-dev pkg-config && \
  #  rm -rf /var/lib/apt/lists/*

# Establece el directorio de trabajo
WORKDIR /app

# Copia los archivos de tu proyecto al contenedor
COPY . .

# Compila la aplicación en modo desarrollo
RUN cargo build

# Establece el puerto expuesto por la aplicación
EXPOSE 5050

# Comando para ejecutar la aplicación
CMD ["cargo", "run"]
