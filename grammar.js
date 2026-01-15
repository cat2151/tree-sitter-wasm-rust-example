module.exports = grammar({
  name: 'chordprog',

  rules: {
    progression: $ => seq(
      $.note,
      repeat(seq('-', $.note))
    ),

    note: $ => choice(
      'C',
      'D',
      'E',
      'F',
      'G',
      'A',
      'B'
    )
  }
});
