require "set"

def main
  data = $stdin.read.chars.flat_map { parse_nibble(_1) }
  version_total, value = decode_packet(data)
  puts "part 1: #{version_total}"
  puts "part 2: #{value}"
end

def decode_packet(data)
  version = consume(data, 3)
  type_id = consume(data, 3)
  if type_id == 4
    return [version, decode_literal(data)]
  end

  length_type_id = data.shift
  versions = []
  values = []
  if length_type_id == 0
    total_length = consume(data, 15)
    stop = data.size - total_length
    while data.size > stop
      a, b = decode_packet(data)
      versions << a
      values << b
    end
  else
    total_packets = consume(data, 11)
    total_packets.times do |i|
      a, b = decode_packet(data)
      versions << a
      values << b
    end
  end

  value =
    case type_id
    when 0
      values.sum
    when 1
      values.inject { |a, b| a * b }
    when 2
      values.min
    when 3
      values.max
    when 5
      raise "uh oh" unless values.size == 2
      values.first > values.last ? 1 : 0
    when 6
      raise "uh oh" unless values.size == 2
      values.first < values.last ? 1 : 0
    when 7
      raise "uh oh" unless values.size == 2
      values.first == values.last ? 1 : 0
    end

  [version + versions.sum, value]
end

def decode_literal(data)
  res = 0
  loop do
    flag = data.shift
    val = consume(data, 4)
    res = (res << 4) + val
    break if flag == 0
  end
  res
end

def consume(data, bits)
  res = 0
  bits.times do
    res = (res << 1) | data.shift
  end
  res
end

def parse_nibble(c)
  n = c.to_i(16)
  [n>>3, n>>2, n>>1, n].map { _1 & 0x1 }
end

main
