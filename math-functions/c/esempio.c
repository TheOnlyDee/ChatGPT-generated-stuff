//Per utilizzare queste funzioni in un altro programma, è sufficiente includere l'intestazione della libreria e richiamare le funzioni come se fossero funzioni del programma stesso. 
//Ad esempio:

#include "math_functions.h"

int main(int argc, char** argv) {
  double x = 2.0;
  double y = 3.0;

  printf("Radice quadrata di %f: %f\n", x, sqrt(x));
  printf("Potenza di %f elevato a %f: %f\n", x, y, pow(x, y));
  printf("Logaritmo di %f: %f\n", x, log(x));

  return 0;
