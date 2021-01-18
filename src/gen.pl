#!/usr/bin/perl

use File::Find;

$acc="";

$llvm_path="./llvm-sys.rs/src";
@files;

find( 
    sub { push @files, $File::Find::name unless -d; }, 
    $llvm_path
);

sub xform_file {
    my ($fname) = @_;
    open(fh, '<', "$fname") or  die $!;
    $noline = "";
    while($line = <fh>) {
      $line =~ s/::libc::c_char/i8/g;
      $line =~ s/::libc::c_uint/u32/g;
      $line =~ s/::libc::size_t/usize/g;
      $line =~ s/\/\/.*\n//g;
      $line =~ s/\t+/ /g;
      $line =~ s/ +/ /g;
      $line =~ s/\n//g;
      $line =~ s/^\s+//g;
      $noline .= $line;
    }
    while($noline =~ /pub fn LLVM(\w+)\((.*?)\)\s*(->\s*(.*?))?;/g) {
      my $name = $1;
      my $args = $2;
      my $ret = $4;
      $acc .= "$name";
      @call = split(",", $args);
      foreach $val (@call) {
        $val =~ /\s*(\w+):\s*(.*)/;
        $acc .= "($2)";
      }
      $acc .= "\[$ret\]\n";
    }
}

foreach $file (@files) {
  if ($file =~ /(.+?)\.rs/) {
    xform_file ($file);
  }

}

print $acc;