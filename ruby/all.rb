ruby_root = File.dirname(__FILE__)
input_root = File.join(ruby_root, "..", "input")

Dir["#{ruby_root}/day*.rb"].each do |f|
  f =~ /day(\d+)/
  daynum = $1
  puts "------", "day #{daynum}"
  system "ruby", f, in: File.open(File.join(input_root, daynum))
end
