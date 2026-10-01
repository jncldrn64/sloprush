# DESIGN.md: cómo se escribe el código

> **Rol:** normativo sobre el código. Cada principio dice cómo se escribe el código y por qué.
> Manda sobre el código; `docs/REQUIREMENTS.md` manda sobre él. Describe reglas; el
> estado, qué rutas, recursos o funciones existen hoy, va en `docs/ARCHITECTURE.md`.
> **Régimen:** se corrige, y un principio nuevo o cambiado lleva su decisión. **Origen:** plantilla
> 1.0.

Un cambio que rompe un principio de acá se rechaza, lo haya escrito una persona o un modelo. Ante la
duda, se copia la forma del código que ya existe en vez de inventar un idioma nuevo.

## 1. <Principio>

<La regla, en una oración que se pueda comprobar. Después, por qué existe: el caso que la hizo
necesaria, con su número si lo tiene. Si se puede comprobar con un comando, el comando.>

## 2. Los tests

<Cómo se escriben y se corren los tests. Una suite que alguna vez falló de forma intermitente se
corre N veces, no una, y se queda en esa lista después del arreglo.>

## 3. Presentación y registro

<Qué imprime el programa, dónde, y con qué detalle según quién lo invoca. Qué se registra y qué
no. Si el producto tiene interfaz, sus reglas de iconos y colores van acá, no en `CLAUDE.md`.>
