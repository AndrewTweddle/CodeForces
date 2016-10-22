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
    val line = lines.next().toUpperCase()  // case insensitive to simplify ad hoc testing
    val soln = solve(line)
    val answer = soln match {
      case Some(rows) => rows.mkString(scala.compat.Platform.EOL)
      case _ => "Impossible"
    }
    bw.write(answer)
    bw.newLine()
  }

  def charToInt(ch: Char): Int = ch.toInt - 'A'.toInt

  case class LetterAnalysis(dupChar: Char, dupIndex1: Int, dupIndex2: Int)

  def analyze(line: String): LetterAnalysis = {
    val ltrPos = Array.fill[Option[Int]](26)(None)
    @tailrec
    def findDupLetter(pos: Int): LetterAnalysis = {
      val char = line(pos)
      val charIndex = charToInt(char)
      if (ltrPos(charIndex).isDefined) {
        val dupIndex1 = ltrPos(charIndex).get
        LetterAnalysis(char, dupIndex1, pos)
      } else {
        ltrPos(charIndex) = Some(pos)
        findDupLetter(pos + 1)
      }
    }
    findDupLetter(0)
  }


  def solve(line: String): Option[Seq[String]] = {
    val len = line.length
    val analysis = analyze(line)
    val di1 = analysis.dupIndex1
    val di2 = analysis.dupIndex2
    if (di2 == di1 + 1) None else Some(
      {
        val before = line.substring(0, di1)
        val after = line.substring(di2 + 1)
        val right = line.substring(di1, di2)
        val left = after + before
        val leftSplitIndex = (len - right.length) / 2
        val rightSplitIndex = (right.length + 1) / 2
        val bottomLeft = left.substring(0, leftSplitIndex).reverse
        val topLeft = left.substring(leftSplitIndex)
        val topRight = right.substring(0, rightSplitIndex)
        val bottomRight = right.substring(rightSplitIndex).reverse
        Seq(topLeft + topRight, bottomLeft + bottomRight)
      }
    )
  }
}
