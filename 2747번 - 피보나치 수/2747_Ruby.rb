n = gets.chomp.to_i

if(n==0)
    puts 0
elsif(n ==1 || n==2)
    puts 1
else
    a = 1
    b = 1
    c = 0
    (n-2).times do |i|
        c = a+b
        a = b
        b = c
    end
    puts c
end
