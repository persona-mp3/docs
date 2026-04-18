import java.util.HashMap;
import java.util.Map;

public class Doc {
  public static void maps(){
    /* The Map implements the basic map structure but has these specified conventions:
     * HashMap
     * TreeMap
     * LinkedHashedMap
     * The Map is an interface with the above classes provided as sub tools to use
     * interface Map {
     *  public static void HashMap();
     *  public static void TreeMap();
     *  public static void LinkedHashedMap();
     * }
     * 
     * Also, why theres confusion in the air as to why Objects and not primitives are used as K,V pairs in maps
     * it seems to stem from the fact that primitives ie int, double, char do not have the .equals() as you know, maps
     * usually work by comparing and sorting with hash codes, objects provide that property. Using primitives will be somehow
     * "different" but i think it's just the JAVA design but these are three main concepts in this: 
     * 1. Autoboxing
     * 2. Unboxing
     * 3. Wrappers
     *
     *
     * Other Methods include:
     * .values()
     * .keySet()
     * .get()
     * .put()
     * .delete()
     * .size()
     * */

    Map<Integer, String> hashM = new HashMap<Integer, String>(); // returns empty map {}

    // unfortunately, you have to use .put(K, V) everytime
    hashM.put(1, "Sam Sulek");
    hashM.put(2, "Ronnie Coleman");
    hashM.put(3, "Tren Twins");

    int mapSize = hashM.size();
    String checkM = hashM.get(4);

    System.out.printf("Map contains following elements -> %s\n", hashM);
    System.out.printf("Map size using .size() -> %d\n", mapSize);
    System.out.printf("Checking map for non-existent key -> %s\n", checkM);

    /* for (type varName : mapName) -> logic*/
    for (int id : hashM.keySet()){
      System.out.printf("userId %d & userName %s\n", id, hashM.get(id) );
    }

  }
  
  public static void forLoops(){

    Map<Integer, String> studentIds = new HashMap<>();

    studentIds.put(3024, "Sam Sulek");
    studentIds.put(3023, "Teddy Bankz");
    studentIds.put(3022, "Solo Leon");
    
    /*
     * The "for-each" loop follows this syntax:
     * for (type variableName : dataStruct){ 
     *   System.out.printf(type)
     * }
     * 
     * It can also be further modified to something like this ->
     * for (int id : studentIds.keySet()){
     *   System.out.printf("%d -> %s\n", id, studentIds.get(id));
     * }
     * */

    // for (String v: studentIds.values()){
    //   System.out.println(v);
    // }


    System.out.println("Both key and values");
    for (int id : studentIds.keySet()){
      System.out.printf("%d -> %s\n", id, studentIds.get(id));
    }

    String[] brands = {"vujade", "pearledIvory", "numberNine", "vivieneWestwood"};
    System.out.println("printing with native for loop");
    for ( int i=0; i < brands.length; i++) {
      System.out.println(brands[i]);
    }

    System.out.println("printing with forEach loop");
    for (String i: brands) {
      System.out.println(i);
    }


  }

  

  public static void main (String[] args){
    maps();
    // forLoops();

  }
}
