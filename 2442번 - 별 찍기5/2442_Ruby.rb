N = gets.chomp.to_i
a = "*"*200
b = " "*200
(1..N).each do |i|
	puts b[1..(N-i)]+a[N-i+1..N+i-1]
end
