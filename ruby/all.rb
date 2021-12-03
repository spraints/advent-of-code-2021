ruby_root = File.dirname(__FILE__)
input_root = File.join(ruby_root, "..", "input")

(1..25).map(&:to_s).each do |daynum|
  f = "ruby/day#{daynum}.rb"
  next unless File.exist?(f)
  puts "------", "day #{daynum}"
  system "ruby", f, in: File.open(File.join(input_root, daynum))
end
