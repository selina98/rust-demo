 #include<stdio.h> 
 #include<stdlib.h> 
 #include<string.h> 
  
 int main(void) 
 { 
     // this is accessing garbage memory
     char *p ; 
  
     strcat(p, "abc"); 
     printf("\n %s \n", p); 
  
     return 0; 
 } 
