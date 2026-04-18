#include <stdio.h>
#include <stdlib.h>

int get_len(char *str);

char *str_replace_all(char *str, char *search, char *replace) {
  int len_str = get_len(str);
  int len_search = get_len(search);
  int len_replace = get_len(replace);

  char result[len_str + (len_replace * len_search)];

  // so for we want to get n..len_search for each iteration
  for (int i = 0; i < len_str; i++) {
    int next_range = i + len_str;
    for (int j = 0; j < next_range; j++) {

    }
  }
}

int get_len(char *str) {
  int count = 0;
  while (1) {
    if (str[count] == '\0') {
      break;
    }
    count++;
  }

  return count;
}

// int main(int argc, char **argv) {
//   char username[] = "mobby_dick";
//   size_t len = sizeof(username);
//   username[len - 4] = 'K';
//   printf("modified username -> %s\n", username);
//
//   char *player_tag = "scobby_dooby_doo";
//   printf("player_tag_mod -> %s\n", player_tag);
//   printf("first_el -> %c\n", player_tag[0]);
//   // ['s', 'c', 'o', '\0']
//   // *player_tag -> s
//   // size_t total_len = sizeof(*player_tag);
//
//   int start = 0;
//   while (1) {
//     char c = player_tag[start];
//     if (c == '\0') {
//       break;
//     }
//
//     printf("%c is the %dth character\n", c, start);
//
//     start++;
//   }
//
//   // this just returs the ascii code or byte representation
// 	// for s
//   printf("*derefed -> %d\n", *player_tag);
// 	*player_tag = 116;
//   printf("*derefed again-> %d\n", *player_tag);
//   return 0;
// }
