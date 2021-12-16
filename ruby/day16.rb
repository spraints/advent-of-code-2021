require "set"

def main
  data = Stream.new($stdin.read.chars)
  version_total = decode_packet(data)
  puts "part 1: #{version_total}"
end

$iters = 0
def decode_packet(data, indent: "")
  $iters += 1
  #exit 1 if $iters > 20
  #puts "#{indent}[#{data.i}]decode_packet"

  version_sum = data.next(3)
  case type_id = data.next(3)
  when 4
    val = decode_literal(data)
    #puts "#{indent}->literal: #{val}"
  else
    length_type_id = data.next(1)
    if length_type_id == 0
      total_length = data.next(15)
      #puts"#{indent}->#{total_length} bits"
      stop = data.i + total_length
      while data.i < stop
        version_sum += decode_packet(data, indent: indent + "  ")
      end
    else
      total_packets = data.next(11)
      #puts"#{indent}->#{total_packets} packets"
      total_packets.times do |i|
        version_sum += decode_packet(data, indent: indent + "  ")
      end
    end
  end
  version_sum
end

def decode_literal(data)
  res = 0
  loop do
    flag = data.next(1)
    val = data.next(4)
    res = (res << 4) + val
    break if flag == 0
  end
  res
end

class Stream
  def initialize(chars)
    @chars = chars.map { _1.to_i(16) }
    # p @chars.take(3)
    @i = 0
  end

  attr_reader :i

  def inspect
    "<Stream chars:#{@chars.size} i:#{@i}>"
  end

  def next(bits)
    res = 0
    bits.times do
      res = res * 2 + next_bit
    end
    #p read: res, bits: bits
    res
  end

  def next_bit
    i = @i
    @i += 1
    mask = 0x1<<(3-(i % 4))
    x = @chars[i/4] & mask
    res = x == 0 ? 0 : 1
    #p i: i, val: res, mask: mask.to_s(2), c: @chars[i/8]
    res
  end
end

main
