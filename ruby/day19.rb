require "set"

def main
  scanners = $stdin.read.split("\n\n").map { parse_scanner(_1) }
  scanners= scanners.take(3)
  p scanners.first.locs.size
  result = []
  matched = scanners.map { false }
  result.push(scanners[0])
  matched[0] = true
  p matched: scanners[0].id
  num_matched = 1
  until matched.all?
    scanners.each_with_index do |s, i|
      next if matched[i]
      s.each_rotation do |rs|
        if ss = find_shifted(rs, result)
          p matched: ss.id
          result.push(ss)
          matched[i] = true
          break
        end
      end
    end

    now_matched = matched.count { _1 }
    raise "only matched #{num_matched} scanners!" if now_matched == num_matched
    num_matched = now_matched
  end
  #puts "part 1: #{result.beacons.size}"
end

def find_shifted(scanner, choices)
  choices.each do |cs|
    if off = find_offset(cs, scanner)
      p id: scanner.id, off: off, from: cs.id
      return scanner.plus(off)
    end
  end
  nil
end

def find_offset(reference, floating)
  mina, locsa = reference.normalize
  minb, locsb = floating.normalize
  puts "COMPARISON"
  puts "#{reference.id} (mins = #{mina.inspect})",
    reference.locs.zip(locsa).map { |a,b| sprintf "%20s -> %20s", a.inspect, b.inspect }
  puts "#{floating.id} (mins = #{minb.inspect})",
    floating.locs.zip(locsb).map { |a,b| sprintf "%20s -> %20s", a.inspect, b.inspect }
  match = locsa & locsb
  puts "MATCHED #{match.size}"
  return nil if match.size < 12
  mina.zip(minb).map { _1 - _2 }
end

def parse_scanner(s)
  id, *locs = s.lines.map(&:strip)
  #id =~ /scanner (\d+)/ or raise "boom"
  #id = $1
  locs = locs.map { _1.split(",").map(&:to_i) }
  Scanner.new(id: id, locs: locs)
end

class Scanner
  def initialize(id:, locs:)
    @id = id
    @locs = locs
  end

  attr_reader :id, :locs

  def plus(offset)
    dx, dy, dz = offset
    Scanner.new(id: "#{id} offset by #{offset.inspect}",
                locs: locs.map { |x,y,z| [x+dx, y+dy, z+dz] })
  end

  def normalize
    min = locs.inject { |a, b| a.zip(b).map(&:min) }
    adj = locs.map { |l| l.zip(min).map { _1 - _2 } }
    [min, adj]
  end

  def each_rotation(&block)
    _each_z_rotation(self, &block)
    _each_z_rotation(self.plus_y_to_plus_z, &block)
    _each_z_rotation(self.plus_x_to_plus_z, &block)
    _each_z_rotation(self.neg_z_to_plus_z, &block)
    _each_z_rotation(self.neg_y_to_plus_z, &block)
    _each_z_rotation(self.neg_x_to_plus_z, &block)
  end

  def plus_y_to_plus_z
    rotate_x("with +y rotated to +z, ", 3)
  end

  def plus_x_to_plus_z
    rotate_y("with +x rotated to +z, ", 1)
  end

  def neg_z_to_plus_z
    rotate_x("with -z rotated to +z, ", 2)
  end

  def neg_y_to_plus_z
    rotate_x("with -y rotated to +z, ", 1)
  end

  def neg_x_to_plus_z
    rotate_y("with -x rotated to +z, ", 3)
  end

  def _each_z_rotation(s)
    yield s
    3.times { yield(s = s.rotate_z("", 1)) }
  end

  #     z
  #     |
  #(-3, |0 (1,3)
  # 1)  |
  #  3  |
  # ----+----y
  #     |  1 (3,-1)
  #     |
  #    2|(-1,-3)
  #     |
  # (+x coming towards me)
  def rotate_x(label, steps)
    _xform("(#{label}rotated around x #{steps} times)") { |x,y,z|
      case steps
      when 1
        [x, z, -y]
      when 2
        [x, -y, -z]
      when 3
        [x, -z, y]
      end
    }
  end

  #     x
  #     |
  #(-3, |0 (1,3)
  # 1)  |
  #  3  |
  # ----+----z
  #     |  1 (3,-1)
  #     |
  #    2|(-1,-3)
  #     |
  # (+y coming towards me)
  def rotate_y(label, steps)
    _xform("(#{label}rotated around y #{steps} times)") { |x,y,z|
      case steps
      when 1
        [-z, y, x]
      when 2
        [-x, y, -z]
      when 3
        [z, y, -x]
      end
    }
  end

  #     y
  #     |
  #(-3, |0 (1,3)
  # 1)  |
  #  3  |
  # ----+----x
  #     |  1 (3,-1)
  #     |
  #    2|(-1,-3)
  #     |
  # (+z coming towards me)
  def rotate_z(label, steps)
    _xform("(#{label}rotated around z #{steps} times)") { |x,y,z|
      case steps
      when 1
        [y, -x, z]
      when 2
        [-x, -y, z]
      when 3
        [-y, x, z]
      else
        raise "boom"
      end
    }
  end

  def _xform(label, &block)
    Scanner.new(id: "#{id} #{label}", locs: locs.map(&block))
  end
end

<<EX

--- scanner 0 ---
0,2
4,1
3,3

--- scanner 1 ---
-1,-1
-5,0
-2,1

normalized: (point - minimums)
--- scanner 0 --- minx:0, miny:1
0,1
4,0
3,2

--- scanner 1 --- minx:-5 miny:-1
4,0
0,1
3,2

position of 1 (from 0): 5,2
  mins[0] - mins[1]


EX

main
