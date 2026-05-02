.TH CATOPY 1 "mayo de 2026" "catopy 0.2.0" "Comandos de Usuario"
.SH NOMBRE
catopy \- copiar contenido de archivos al portapapeles del sistema de forma segura
.SH SINOPSIS
.B catopy
.RI [ OPCIONES ] " ARCHIVO"
.SH DESCRIPCIÓN
.B catopy
lee texto de un archivo y lo coloca en el portapapeles del sistema.
Incluye controles de tamaño para evitar copiar accidentalmente archivos muy grandes.
En Linux, la transferencia del portapapeles usa una breve espera para mejorar la confiabilidad en procesos de corta duración.
.SH OPCIONES
.TP
.BR --head " " N
Copia solo las primeras N líneas.
.TP
.BR --tail " " N
Copia solo las últimas N líneas.
.TP
.B --force
Omite el control de tamaño de archivo.
.TP
.BR --max-bytes " " TAMAÑO
Tamaño máximo de archivo antes de rechazar la copia. Admite sufijos K, M, G.
.TP
.B --no-color
Desactiva la salida con color ANSI.
.TP
.BR -h , " " --help
Imprime información de ayuda.
.TP
.BR -V , " " --version
Imprime información de versión.
.SH ARCHIVO DE CONFIGURACIÓN
Archivo de configuración opcional:
.I ~/.catopy.rc
.PP
Claves soportadas:
.TP
.B behave_as_cat
Si es verdadero, imprime el contenido copiado en stdout.
.TP
.B warning_size_limit
Límite de tamaño de archivo predeterminado.
.TP
.B default_head
Valor predeterminado para --head.
.TP
.B default_tail
Valor predeterminado para --tail.
.SH EJEMPLOS
.TP
Copiar archivo completo:
.B catopy notas.txt
.TP
Copiar primeras 20 líneas:
.B catopy --head 20 /var/log/messages
.TP
Copiar últimas 100 líneas:
.B catopy --tail 100 app.log
.TP
Anular límite:
.B catopy --max-bytes 5M volcado.txt
.SH AUTOR
Mantenedores de catopy.
.SH LICENCIA
Licencia GPLv3.
