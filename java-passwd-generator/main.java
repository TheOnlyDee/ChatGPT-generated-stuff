import java.util.Random;

public class PasswordGenerator {
  public static void main(String[] args) {
    Random rng = new Random();
    StringBuilder password = new StringBuilder();
    for (int i = 0; i < 25; i++) {
      int characterType = rng.nextInt(3);
      char character;
      if (characterType == 0) {
        // Genera una lettera maiuscola
        character = (char) ('A' + rng.nextInt(26));
      } else if (characterType == 1) {
        // Genera una lettera minuscola
        character = (char) ('a' + rng.nextInt(26));
      } else {
        // Genera un numero
        character = (char) ('0' + rng.nextInt(10));
      }
      password.append(character);
    }
    System.out.println("La tua password è: " + password.toString());
  }
}
