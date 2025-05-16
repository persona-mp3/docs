import java.util.Map;
import java.util.HashMap;
import java.util.ArrayList;

public class TwoSumR{

  public static void main(String[] args){
    // int[] arr = {1, 2, 3, 4, 5};
    int[] arr = {3, 3};
    // int[] arr = {2, 7, 11, 15}; --> 9 
    int target = 6;

    ArrayList<Integer> idxs = TwoSumHelper(arr, target);
    System.out.println(idxs);
  }

  public static ArrayList<Integer> TwoSumHelper(int[] arr, int target){

    Map<Integer, Integer> hash = new HashMap<>();
    ArrayList<Integer> result = new ArrayList<>();
    

    for (int i=0; i < arr.length; i++ ){
      int targetN = target - arr[i];

      /* Edge case here -> hashMaps in java do not allow duplicate values so i'll need to reverse the order*/

      // if hash.containsKey(targetN)
      if (hash.containsKey(targetN) && i != hash.get(targetN)) {
        System.out.printf("%d ** %d\n", targetN, i);
        result.add(i);
        result.add(hash.get(targetN));

        return result;
      }
      hash.put(arr[i], i);
    }
    
    System.out.println("Target not found");
    return new ArrayList<>();

  }
}
