# Introducción a MPC y TSS

La computación multipartita segura (SMPC/MPC) permite que varios participantes calculen de manera colaborativa una función acordada previamente sin la necesidad de un tercero de confianza. Este concepto se originó a partir del problema del millonario del profesor Andrew Yao y su solución mediante criptografía en 1982. El esquema, es una interacción entre dos personas, podía descubrir quién era más rico sin revelar su riqueza real. Permite que los usuarios colaboren en los cálculos entre sí sin revelar ninguna información confidencial. Desde entonces MPC ha evolucionado hasta convertirse en una rama importante de la criptografía moderna.

# Ejemplo Motivador

Imagina que un grupo de 3 investigadores está trabajando en un proyecto conjunto y cada uno ha desarrollado un algoritmo innovador con resultados distintos. Los investigadores quieren colaborar para descubrir cuál de los algoritmos es el más eficiente, es decir, cuál tiene el menor tiempo de ejecución. Sin embargo, cada investigador considera que los detalles de su propio trabajo son confidenciales y no quiere revelar sus resultados exactos a los demás. Para solucionar esto, necesitan determinar cuál es el algoritmo más rápido sin exponer los tiempos de ejecución de cada uno.

El problema puede ser representado por la función:
$$F(t_1, t_2, t_3) = min(t_1, t_2, t_3)$$
​donde $t_1, t_2, t_3$ son los tiempos de ejecución de cada uno de los algoritmos de los investigadores.

Una solución sencilla sería que todos revelaran sus tiempos de ejecución a un tercero que pudiera calcular el valor mínimo. Sin embargo, como no confían en ninguna entidad externa para manejar estos datos confidenciales, necesitan una forma de encontrar el resultado sin revelar sus tiempos a nadie más.

Usando un protocolo de computación multipartidaria (MPC), los investigadores podrían colaborar para determinar cuál es el algoritmo más rápido. El protocolo garantizaría que cada uno contribuya con su tiempo de ejecución a la función, pero sin que nadie pueda inferir el tiempo de los demás, más allá de saber cuál fue el más rápido. Así, logran comparar sus algoritmos sin comprometer la confidencialidad de sus investigaciones.