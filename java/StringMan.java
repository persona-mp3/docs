public class StringMan{
  public static void Basics(){
    String johnson = "Johnson Cotton Buds";

    int strLen = johnson.length();
    int findChar = johnson.charAt(0);
    int findIndex = johnson.indexOf("s") // returns the first occurence of the chatracter

    System.out.printf("%d, %c, %d", strLen, findChar, findIndex)
  }

  public static void main(String[] args) {
    Basics();

  }
}
