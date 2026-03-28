/// <reference path='fourslash.ts' />

//// enum Demo {
////     /*Emoji*/Emoji = '🍎',
////     /*Hebrew*/Hebrew = 'תפוח',
////     /*Chinese*/Chinese = '苹果',
////     /*Japanese*/Japanese = 'りんご',
//// }

verify.quickInfoAt("Emoji", '(enum member) Demo.Emoji = "🍎"');
verify.quickInfoAt("Hebrew",'(enum member) Demo.Hebrew = "תפוח"');
verify.quickInfoAt("Chinese", '(enum member) Demo.Chinese = "苹果"');
verify.quickInfoAt("Japanese",'(enum member) Demo.Japanese = "りんご"');
