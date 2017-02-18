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
    val counts = line.split(" ").map(_.toInt)
    val (c1, c2) = (counts(0), counts(1))
    var firstWordSet = (1 to c1).map(_ => lines.next()).toSet
    val sharedWordCount = (1 to c2).count(_ => firstWordSet.contains(lines.next()))
    val soln = solve(c1 - sharedWordCount, c2 - sharedWordCount, sharedWordCount)
    bw.write(if (soln) "YES" else "NO")
    bw.newLine()
  }

  // Answer is whether first player can win
  def solve(unique1: Int, unique2: Int, shared: Int): Boolean = {
    if (unique1 == unique2) {
      // If odd number of shared words, then first person gets to say one more shared word before they run out, to win
      // Else second can always answer the first
      (shared % 2 != 0)
    } else unique1 > unique2  // keep saying shared words, which decreases both players'
  }
}
