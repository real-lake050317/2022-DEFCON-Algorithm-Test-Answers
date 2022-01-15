number_string = gets.chomp
number = []
number_string.each_char {|char| number << char.to_i}
if number.length == 1
  number << number[0]
  number[0] = 0
end
i = 0
loop do 
  new_number = []
  new_number[0] = number[1]
  new_number[1] = (number[0] + number[1])%10
  number = new_number
  i = i + 1
  break if (number[0]*10 + number[1]) == number_string.to_i
end
puts "#{i}"
