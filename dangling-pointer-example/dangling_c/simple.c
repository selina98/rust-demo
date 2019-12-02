 #include<stdio.h> 
 #include<stdlib.h> 
 #include<string.h> 
  
 int main(void) 
 { 
    char **strPtr;
    char *str = "Hello!";

    strPtr = &str;
    free(str);
    //strPtr now becomes a dangling pointer

    printf("%s", *strPtr);
 } 
