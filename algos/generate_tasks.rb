#!/usr/bin/env ruby
(0...50_000).each do |n| puts "Task #{n},#{Random::rand(100) + 1}"; end
