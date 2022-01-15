using System;

public class Hello {
	public static void Main() {
  
		int n = int.Parse(Console.ReadLine());
    
		string s = "";
    
		for(int i = n - 1; i >= 0; i--) {
    
			for(int j = 0; j < n; j++) {
				s += j < i ? " " : j == n - 1 ? "*" : "**";
			}
      
			Console.WriteLine(s);
			s = "";
		}
	}
}
