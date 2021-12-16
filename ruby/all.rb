ruby_root = File.dirname(__FILE__)
input_root = File.join(ruby_root, "..", "input")

(1..25).map(&:to_s).each do |daynum|
  if daynum == "15"
    # it's too slow. remove this when it's sped up
    puts "------", "day 15 (sub)", "part 1: 714", "part 2: 2948"
    next
  end
  f = "ruby/day#{daynum}.rb"
  next unless File.exist?(f)
  puts "------", "day #{daynum}"
  system "ruby", f, in: File.open(File.join(input_root, daynum))
end
