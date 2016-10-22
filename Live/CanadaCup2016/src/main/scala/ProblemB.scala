import java.io._
import scala.io.Source

import scala.annotation.tailrec

object ProblemB {
  def main(args: Array[String]): Unit = processStdInOut()

  def processStdInOut(): Unit = {
    val src = Source.fromInputStream(System.in)
    processFromSource(src)
  }

  def processFromSource(src: Source): Unit = {
    try {
      val bw = new BufferedWriter(new OutputStreamWriter(System.out));
      try {
        val lines = src.getLines()
        processLines(lines, bw)
      } finally {
        bw.flush()
      }
    } finally {
      src.close();
    }
  }

  // Standard code above, custom code below

  def processLines(lines: Iterator[String], bw: BufferedWriter): Unit = {
    val line = lines.next()
    val len = line.length
    val seat = line(len - 1)
    val row = line.substring(0, len - 1).toLong
    val soln = solve(row, seat)
    bw.write(soln.toString)
    bw.newLine()
  }

  def solve(row: Long, seat: Char): Long = {
    val charMap = "fedabc".zipWithIndex.toMap
    val seatOrderInRow = charMap(seat) + 1
    val setOfRowPairs = (row - 1) / 4
    val orderInRowOf4 = (row - 1) % 2
    (setOfRowPairs * 16) + orderInRowOf4 * 7 + seatOrderInRow
  }
}
