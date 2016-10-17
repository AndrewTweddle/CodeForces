import java.io._
import scala.io.Source

import scala.annotation.tailrec

object ProblemC {
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
    val strings = line.split(" ")
    val b = strings(0).toLong
    val d = strings(1).toLong
    val s = strings(2).toLong
    val soln = solve(b, d, s)
    bw.write(soln.toString)
    bw.newLine()
  }

  def solve(b: Long, d: Long, s: Long): Long = {
    // rebase time, so that at least as many of first meal as any other, and call that breakfast...
    if (d > b && d >= s) solve(d, s, b)  // Measure from before dinner on the first day
    else if (s > d && s >= b) solve(s, b, d)  // Measure from before supper on the first day
    else math.max(b - d - 1, 0) + math.max(b - s - 1, 0)
        // don't count last dinner and/or supper as missed, but beware that b, d and s could be equal
  }
}
