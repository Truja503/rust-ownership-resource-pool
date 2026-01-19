🦀 Rust Ownership & Resource Management — Progress Log

Resource Pool (Ownership-Oriented Design)

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
