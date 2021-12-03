class BitTree
  def initialize
    @n0 = @n1 = 0
  end

  attr_reader :n0, :n1, :ones, :zeros

  def one
    @n1 += 1
    @ones ||= BitTree.new
  end

  def zero
    @n0 += 1
    @zeros ||= BitTree.new
  end

  def o2(res = 0)
    return res if @n0 + @n1 == 0
    if @n0 > @n1
      @zeros.o2(res*2)
    else
      @ones.o2(res*2 + 1)
    end
  end

  def co2(res = 0)
    return res if @n0 + @n1 == 0
    if @n0 > @n1
      if @n0 == 1
        @zeros.o2(res*2)
      else
        @ones.co2(res*2 + 1)
      end
    else
      if @n1 == 1 && @n0 == 0
        @ones.o2(res*2 + 1)
      else
        @zeros.co2(res*2)
      end
    end
  end
end

bit_counts = []
bit_count_tree = BitTree.new

readlines(chomp:true).each do |line|
  t = bit_count_tree
  line.each_char.with_index do |c, i|
    bit_counts[i] ||= [0,0]
    case c
    when "0"
      bit_counts[i][0] += 1
      t = t.zero
    when "1"
      bit_counts[i][1] += 1
      t = t.one
    else
      raise "boom #{line.inspect} #{c.inspect}/#{i}"
    end
  end
end

gamma = bit_counts.map { |a,b| a > b ? 0 : 1 }.inject { |n, b| n*2 + b }
epsilon = bit_counts.map { |a,b| a > b ? 1 : 0 }.inject { |n, b| n*2 + b }
puts "part 1: #{gamma * epsilon}"

puts "part 2: #{bit_count_tree.o2 * bit_count_tree.co2}"
