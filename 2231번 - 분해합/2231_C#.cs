using System;
using System.Collections.Generic;

namespace BaekjoonStudy
{
    class Program
    {
        static int input;
        static bool isExist;

        static void Main()
        {
            input = Convert.ToInt32(Console.ReadLine());

            for (int i = 0; i < input; i++)
            {
                int temp = i;
                int result = i;

                while (temp > 0)
                {
                    result += temp % 10;
                    temp /= 10;
                }

                if (result == input)
                {
                    Console.WriteLine(i);
                    isExist = true;
                    break;
                }
            }

            if(!isExist)
                Console.WriteLine(0);
        }        
    }
}
