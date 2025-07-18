# Lab2-Graficas
# Conway's Game of Life - Rust + Raylib

Este es el **Laboratorio #2** que implementa el famoso **Conway’s Game Of Life** utilizando el lenguaje **Rust** y la librería gráfica **Raylib**, permitiendo visualizar en tiempo real el avance de las generaciones de células.

## 🧬 ¿Qué es el Juego de la Vida?
El **Juego de la Vida** es un autómata celular ideado por John Conway. Consiste en una cuadrícula de celdas que siguen reglas simples:
- Células vivas sobreviven si tienen 2 o 3 vecinas vivas.
- Células muertas reviven si tienen exactamente 3 vecinas vivas.
- En cualquier otro caso, las células mueren o permanecen muertas.

Aunque las reglas son simples, emergen comportamientos complejos como patrones estables, osciladores, "spaceships" y "glider guns".

---

## 📁 Estructura del Proyecto

```bash
src/
├── main.rs        # Ciclo principal del juego y renderizado general
├── grid.rs        # Lógica de simulación, evolución y visualización de la grilla
├── patterns.rs    # Definición y generación de patrones clásicos como gliders y spaceships
```
## 🎮 Requisitos
- Rust
- Raylib
  ```bash
  cargo add raylib
  ```
- Librería rand
    ```bash
    cargo add rand
    ```
    
## 🚀 Cómo ejecutar
    ```bash
    cargo run
    ```
## 📷 Demostración
![Conway's Game of Life mov](https://github.com/user-attachments/assets/8bba166a-ce8e-44cc-8c9e-bf7966353d2d)

## 📜 Créditos
- Implementación realizada como parte del laboratorio de programación universitaria.
- Librería gráfica: Raylib
- Inspirado en el clásico Conway's Game of Life.

    
