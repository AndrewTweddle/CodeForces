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
    val n = lines.next().toLong
    val log2 = math.log(2)
    val rounds = math.ceil(math.log(n) / log2) + 0.1
    // note: bump it slightly in case of round errors like a ?.999999 "integer"
    bw.write(rounds.toLong.toString)
    bw.newLine()
  }
}
