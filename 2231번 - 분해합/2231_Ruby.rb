def d(n)
  return n + n.to_s.split('').map(&:to_i).sum
end

n = gets.chomp.to_i

for i in 1...n do 
  v = d(i)
  if v == n
    puts i 
    return 
  end
end
puts 0
