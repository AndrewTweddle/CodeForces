import java.io._
import scala.io.Source

import scala.annotation.tailrec

object ProblemX {
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

    // val line = lines.next()
    // val strings = line.split(" ")
    // val soln = solve(...)
    //bw.write(s"<solution here>")
    bw.newLine()
  }
}
