using System;

public class Hello {
	public static void Main() {
  
		int n = int.Parse(Console.ReadLine());
		int a = 0, b = 1, c = 1;
    
		if(n < 2)
			Console.WriteLine(n);
		else {
			for(int i = 0; i < n - 1; i++) {
				c = b;
				b = a + b;
				a = c;
			}
		    Console.WriteLine(b);
		}
    
	}
	static int Fibonacci(int n) {
		
    if(n < 2)
			return n;
		else
			return Fibonacci(n - 1) + Fibonacci(n - 2);
	}
}
