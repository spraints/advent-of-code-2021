require "set"

def main
  lines = $stdin.readlines.map { |l| eval(l) }
  res = lines.inject { |a, b| reduce([a,b]) }
  p res
  puts mag(res)
  #p result: reduce([[[[[9,8],1],2],3],4])
  #puts "------"
  #p result: reduce([7,[6,[5,[4,[3,2]]]]])
  #puts "------"
  #p result: reduce([[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]])
  #puts "------"
  #p result: reduce([[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]])
end

def mag(sn)
  case sn
  when Array
    a, b = sn
    3*mag(a) + 2*mag(b)
  else
    sn
  end
end

def reduce(sn)
  p input: sn
  sn = convert(sn)
  loop do
    next if explode(sn)
    next if split(sn)
    return unconvert(sn)
  end
end

def convert(sn, depth: 0, res: [])
  case sn
  when Array
    convert(sn[0], depth: depth+1, res: res)
    convert(sn[1], depth: depth+1, res: res)
  else
    res << [sn, depth]
  end
  res
end

def unconvert(sn)
  p unconvert: sn
  res = [[]]
  while n = sn.shift
    #p res: res, n: n
    while n[1] < res.size
      loop do
        a = res.pop
        res.last.push(a)
        break unless res.last.size == 2
      end
    end
    while n[1] > res.size
      res.push([])
    end
    res.last.push(n[0])
    while res.last.size == 2 && res.size > 1
      a = res.pop
      res.last.push(a)
    end
  end
  until res.size == 1
    a = res.pop
    res.last.push(a)
  end
  res.pop
end

def explode(sn)
  (0..sn.size-1).each do |i|
    if sn[i][1] > 4 && sn[i+1][1] == sn[i][1]
      p1 = sn[i]
      p2 = sn.delete_at(i+1)
      sn[i] = [0, p1[1]-1]
      if i > 0
        n1 = sn[i-1]
        n1[0] += p1[0]
      end
      if n2 = sn[i+1]
        n2[0] += p2[0]
      end
      return true
    end
  end
  false
end

def split(sn)
  i = sn.index { |x,d| x >= 10 }
  return false if i.nil?
  x, d = sn[i]
  xa = x/2
  xb = x-xa
  sn[i] = [xa, d+1]
  sn.insert(i+1, [xb, d+1])
  true
end

main
