class Array
  def sum
    self.inject(0) {|s, x| s + x }
  end
end

def std_deviation ary
  mean = ary.sum / ary.length.to_f
  Math.sqrt(ary.map {|x| (x - mean)**2}.sum / ary.length.to_f)
end

puts std_deviation(ARGV.map(&:to_f))
