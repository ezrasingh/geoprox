# -*- encoding: utf-8 -*-

=begin
#geoprox-server

#Geoprox server implementation providing a HTTP API for geospatial queries and position tracking

The version of the OpenAPI document: 0.2.0
Contact: singhezra@gmail.com
Generated by: https://openapi-generator.tech
Generator version: 7.7.0

=end

$:.push File.expand_path("../lib", __FILE__)
require "geoprox_client/version"

Gem::Specification.new do |s|
  s.name        = "geoprox_client"
  s.version     = GeoproxClient::VERSION
  s.platform    = Gem::Platform::RUBY
  s.authors     = ["OpenAPI-Generator"]
  s.email       = ["singhezra@gmail.com"]
  s.homepage    = "https://github.com/ezrasingh/geoprox/contrib/client-sdk/ruby/"
  s.summary     = "geoprox-server Ruby Gem"
  s.description = "Geoprox server implementation providing a HTTP API for geospatial queries and position tracking"
  s.license     = "Unlicense"
  s.required_ruby_version = ">= 2.7"
  s.metadata    = {}

  s.add_runtime_dependency 'typhoeus', '~> 1.0', '>= 1.0.1'

  s.add_development_dependency 'rspec', '~> 3.6', '>= 3.6.0'

  s.files         = `find *`.split("\n").uniq.sort.select { |f| !f.empty? }
  s.test_files    = `find spec/*`.split("\n")
  s.executables   = []
  s.require_paths = ["lib"]
end
