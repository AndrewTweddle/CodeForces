import java.io._

object ProblemA {
  def main(args: Array[String]): Unit = processStdInOut()

  def processStdInOut(): Unit = {
    val src = io.Source.fromInputStream(System.in)
    try {
      val bw = new BufferedWriter(new OutputStreamWriter(System.out));
      try {
        val lines = src.getLines()
        processLines(lines, bw)
      } finally {
        bw.flush()
        bw.close()
      }
    } finally {
      src.close();
    }
  }

  // Standard code above, custom code below

  def processLines(lines: Iterator[String], bw: BufferedWriter): Unit = {
    val day1 = lines.next()
    val day2 = lines.next()
    val hasSolution = solve(day1, day2)
    if (hasSolution) { bw.write("YES") } else {bw.write("NO") }
    bw.newLine()
  }

  def solve(day1: String, day2: String): Boolean = {
    val dayNames = List("monday", "tuesday", "wednesday", "thursday", "friday", "saturday", "sunday")
    def getDayIndex(dayName: String): Int = dayNames.indexOf(dayName)
    val day1Index = getDayIndex(day1)
    val day2Index = getDayIndex(day2)
    val dayOffset = (7 + day2Index - day1Index) % 7  // Add 7 to prevent negative remainders
    val monthLengths = List(28, 30, 31)
    monthLengths.exists(dayOffset == _ % 7)
  }
}
