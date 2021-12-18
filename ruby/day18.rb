require "set"

def main
  lines = $stdin.readlines.map { |l| eval(l) }
  res = lines.inject { |a, b| sum(a, b) }
  puts "part 1: #{mag(res)}"

  max = 0
  (0..lines.size-1).each do |i|
    (0..lines.size-1).each do |j|
      next if i == j
      m = mag(sum(lines[i], lines[j]))
      max = m if m > max
    end
  end
  puts "part 2: #{max}"
end

def sum(a, b)
  tree = Tree.new(pair: [a, b])
  reduce(tree)
  tree.to_a
end

def reduce(tree)
  loop do
    #p tree_before: tree
    next if explode(tree)
    next if tree.split
    break
  end
  #p tree_after: tree
end

def explode(obj, depth: 0)
  case
  when obj.leaf?
    false
  when obj.l.leaf? && obj.r.leaf?
    return false if depth < 4
    #p explode: obj, l: obj.left_neighbor, r: obj.right_neighbor
    obj.left_neighbor.add(obj.l)
    obj.right_neighbor.add(obj.r)
    obj.delete
    true
  else
    explode(obj.l, depth: depth+1) || explode(obj.r, depth: depth+1)
  end
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

class Tree
  def initialize(pair:, parent: nil)
    @l = make_node(pair[0])
    @r = make_node(pair[1])
    @parent = parent
  end

  attr_reader :l, :r, :parent

  def pair?; true; end
  def leaf?; false; end

  def inspect
    [@l, @r].inspect
  end

  def to_a
    [@l.to_a, @r.to_a]
  end

  def split
    l.split || r.split
  end

  def left_neighbor
    case
    when @parent.nil?
      NullNode.new
    when self == @parent.l
      @parent.left_neighbor
    else
      @parent.l.right_leaf
    end
  end

  def right_neighbor
    case
    when @parent.nil?
      NullNode.new
    when self == @parent.r
      @parent.right_neighbor
    else
      @parent.r.left_leaf
    end
  end

  def left_leaf
    l.left_leaf
  end

  def right_leaf
    r.right_leaf
  end

  def delete
    @parent.replace(self, Leaf.new(value: 0, parent: @parent))
  end

  def replace(child, new_child)
    if child == l
      @l = new_child
    else
      @r = new_child
    end
  end

  private

  def make_node(v)
    case v
    when Array
      Tree.new(pair: v, parent: self)
    else
      Leaf.new(value: v, parent: self)
    end
  end
end

class Leaf
  def initialize(value:, parent:)
    @value = value
    @parent = parent
  end

  attr_reader :value, :parent

  def pair?; false; end
  def leaf?; true; end

  def left_leaf; self; end
  def right_leaf; self; end

  def add(other)
    @value += other.value
  end

  def split
    return false if value < 10
    a = value / 2
    b = value - a
    @parent.replace(self, Tree.new(pair: [a, b], parent: @parent))
  end

  def inspect
    @value.inspect
  end

  def to_a
    @value
  end
end

class NullNode
  def add(*); end

  def inspect
    "(null)"
  end
end

main
