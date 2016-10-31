import java.io._
import scala.io.Source

import scala.annotation.tailrec

object ProblemA {
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

  case class Accumulator(maxRun: Int, currRun: Int, isLastAVowel: Boolean)

  def processLines(lines: Iterator[String], bw: BufferedWriter): Unit = {
    val line = lines.next()
    val consonantBlocks = line.split("[AEIOUY]")
    val maxJump = if (consonantBlocks.isEmpty) 1 else consonantBlocks.map(_.length + 1).max
    bw.write(maxJump.toString)
    bw.newLine()
  }
}
