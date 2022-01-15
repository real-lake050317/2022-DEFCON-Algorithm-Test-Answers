using System;
using System.IO;
using System.Text;

namespace functionthings
{
    class Program
    {
        static void Main(string[] args)
        {
            int t,sub, o, num=Convert.ToInt32(Console.ReadLine()),count=0;
            t = num / 10; o = num % 10;
            while(true)
            {
                count++;
                sub = t;
                t = o;
                o = (sub + t)%10;
                if (num == t * 10 + o) break;
            }
            Console.WriteLine(count);
        }

    }
}
