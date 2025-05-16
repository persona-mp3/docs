import java.io.FileReader;
import java.io.FileNotFoundException;
import java.io.IOException;
import java.util.Scanner;

public class Fileio {

  public static void ReadF(){
    /*
     * 1. Using the buitlin FileReader method from the java.io.FileReader, it takes the path and returns the file in bytes
     * 2. To start reading the file, use reader.read(), kinda straight forward, yk java might not be that bad
     * 3. reader.read() -> int contains the byte value of 
     * 4. Now it's possible to read the EOF, and we want to stop reading, we can do this by using the int value returned
     * 5. if reader.read() -> -1; reader.close()
     *
     * Here's the pseudocode
     * FileReader reader = new FileReader(path/to/workingDir) <i think this method handles opening the file>
     * isEOF = reader.read() <gets input stream in bytes and converts to int>
     * isEOF != -1 continue reading; print((char)line)
     *
     * NOTE:
     * While java just splashes errors all over the screen, its always better to wrap all FileIO operations in 
     * try{} catch{ErrorType err}
     * So include these imports :
     *            java.io.FileNotFoundException
     *            java.io.IOException
     * 
     *
     * To just open a file, you use:
     * File f = new File(path)
     * .exists() -> boolean
     * .createNewFile() -> boolean
     * .canWrite()-> boolean
     * .canRead() -> boolean
     * .getAbsolutePath() -> string
     * .getName() -> string
     * .length() -> file size in bytes
     *
     * .setRead(true)
     * .setWrite(true)
     * .setExec(true)
     *
     * */


    try{
      FileReader reader = new FileReader("./src/Doc.java");

      int line = reader.read(); 
     
      while (line != -1) {
        System.out.print((char)line);
        line = reader.read();
      }

      reader.close();

    } catch(FileNotFoundException e){
      System.out.println("error\n");
      e.printStackTrace();

    } catch(IOException e) {
      e.printStackTrace();
    }
  }


  public static void main(String[] args){
    ReadF();

  }
}

