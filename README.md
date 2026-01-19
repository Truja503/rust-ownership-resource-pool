🦀 Rust Ownership & Resource Management — Progress Log

Este repositorio documenta mi progreso práctico aprendiendo ownership, move semantics, borrowing y Drop en Rust, a través de proyectos pequeños pero conceptualmente exigentes.

No son ejercicios de sintaxis ni ejemplos de tutorial.
Cada proyecto está diseñado para forzar el modelo mental de Rust, especialmente en lo relacionado con ciclo de vida y gestión de recursos.

📌 Proyectos incluidos
🗡️ Project 1 — Inventory with Unique Ownership

Objetivo:
Modelar transferencia de recursos entre inventarios usando ownership real (sin referencias compartidas).

Conceptos trabajados:

move vs copy

&mut y exclusividad

Transferencia explícita de ownership

Scopes y destrucción automática

Drop para observar el ciclo de vida de los objetos

Este proyecto me permitió entender que en Rust el estado no se describe con flags, sino con quién es dueño del dato.

🗡️ Project 3 — Resource Pool (Ownership-Oriented Design)

Objetivo:
Construir un pool que presta recursos temporalmente, sin referencias, usando únicamente movimientos de ownership.

Modelo mental:
Un recurso existe en un solo lugar a la vez:
En el pool → disponible
Fuera del pool → en uso
No se usan:
Flags de estado
Referencias (&T)
Smart pointers

Conceptos trabajados:
Ownership como flujo
Modelado de estados mediante contenedores
RAII y Drop
Scopes como ciclos de vida reales
Manejo explícito de recursos

🧠 Enfoque

Este repositorio no busca “código bonito”, sino claridad conceptual.
La prioridad fue entender por qué Rust funciona así, no solo cómo hacerlo compilar.

La progresión entre proyectos es intencional y refleja cómo fui reemplazando modelos mentales tradicionales por el modelo de Rust.
