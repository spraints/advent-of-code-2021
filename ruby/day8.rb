def main
  input = $stdin.readlines.map { |line| line.split("|").map(&:split) }

  simple_count = input.inject(0) { |count, ab| b = ab[1]; count + b.count { |s| [2,4,3,7].include?(s.size) } }
  puts "part 1: #{simple_count}"

  total = input.map { |line| decode(*line) }.sum
  puts "part 2: #{total}"
end

def decode(hint, code)
  key = {}
  missing = Hash.new { |h, c| h[c] = [] }
  used = Hash.new { |h, c| h[c] = [] }
  hint.each do |s|
    fixed = fix(s)
    case fixed.size
    when 2
      #p fixed => 1
      key[fixed] = 1
    when 4
      #p fixed => 4
      key[fixed] = 4
    when 3
      #p fixed => 7
      key[fixed] = 7
    when 7
      #p fixed => 8
      key[fixed] = 8
    else
      (%w(a b c d e f g) - s.chars).each do |c|
        missing[c] << fixed
      end
    end
    s.chars.each do |c|
      used[c] << fixed
    end
  end
  #p missing

  c, zero = missing.find { |_, v| v.size == 1 && v[0].size == 6 }
  zero = zero.first
  #p zero => 0
  key[zero] = 0
  missing.values.each { |v| v.delete(zero) }

  c, vs = missing.find { |_, v| v.size == 2 && v.any? { |s| s.size == 6 } }
  six = vs.find { |s| s.size == 6 }
  five = vs.find { |s| s.size == 5 }
  #p six => 6
  #p five => 5
  key[six] = 6
  key[five] = 5
  missing.values.each { |v| v.delete(six); v.delete(five) }

  c, vs = missing.find { |_, v| v.any? { |s| s.size == 6 } }
  nine = vs.find { |s| s.size == 6 }
  three = vs.find { |s| s.size == 5 }
  #p nine => 9
  #p three => 3
  key[nine] = 9
  key[three] = 3
  missing.values.each { |v| v.delete(nine); v.delete(three) }

  two = missing.values.flatten.uniq.first
  #p two => 2
  key[two] = 2

  #p missing.sort_by { |k, v| k }.to_h
  #p key.sort_by { |_, v| v }.to_h
  #exit 0

  # 0: 6, missing d     <-- only one missing d.
  # 2: 5, missing b, f
  # 3: 5, missing b, e
  # 5: 5, missing c, e  <-- buddy with 6
  # 6: 6, missing c     <-- only two are missing this.
  # 9: 6, missing e
  # ==> 0
  # ==> 5,6
  #
  
  code.map { |s| key[fix(s)] }.inject(0) { |x,y| x*10+y }
end

def fix(s)
  s.chars.sort.join
end

main
